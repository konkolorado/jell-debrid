use log;
use std::collections::HashMap;
use std::time::Duration;

use mini_moka::unsync::Cache;
use oauth2;
use oauth2::{DeviceAuthorizationResponse, ExtraDeviceAuthorizationFields, TokenResponse};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::base::HttpClient;
use crate::trakt::client::client_utils::async_http_client;
use crate::trakt::structs::{SearchResult, WatchList};

pub struct TraktClient {
    client: HttpClient,
    cache: Cache<String, Result<SearchResult, reqwest::Error>>,
}

#[derive(Debug)]
pub enum MediaType {
    Movie,
    Show,
}

pub struct TraktOAuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoringFields(HashMap<String, serde_json::Value>);

impl ExtraDeviceAuthorizationFields for StoringFields {}
type StoringDeviceAuthorizationResponse = DeviceAuthorizationResponse<StoringFields>;

impl TraktClient {
    pub fn new(token: &str, client_id: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());
        headers.insert("trakt-api-key", client_id.parse().unwrap());
        headers.insert("trakt-api-version", "2".parse().unwrap());
        Self {
            client: HttpClient::new("https://api.trakt.tv", Some(headers)).unwrap(),
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(5 * 60))
                .build(),
        }
    }

    pub async fn oauth2(client_id: &str, client_secret: &str) -> TraktOAuthToken {
        let trakt_client_id = oauth2::ClientId::new(client_id.to_string());
        let trakt_client_secret = oauth2::ClientSecret::new(client_secret.to_string());
        let auth_url = oauth2::AuthUrl::new("https://api.trakt.tv/oauth/device/code".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url =
            oauth2::TokenUrl::new("https://api.trakt.tv/oauth/device/token".to_string())
                .expect("Invalid token endpoint URL");
        let device_auth_url = oauth2::DeviceAuthorizationUrl::new(
            "https://api.trakt.tv/oauth/device/code".to_string(),
        )
        .expect("Invalid device authorization endpoint URL");

        let device_client = oauth2::basic::BasicClient::new(
            trakt_client_id,
            Some(trakt_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_device_authorization_url(device_auth_url)
        .set_auth_type(oauth2::AuthType::RequestBody);

        let details: StoringDeviceAuthorizationResponse = device_client
            .exchange_device_code()
            .unwrap()
            .request_async(&oauth2::reqwest::async_http_client)
            .await
            .expect("Failed to request codes from device auth endpoint");

        println!("{details:?}");
        println!("{:?}", details.device_code().secret());
        println!("{:?}", details.interval());
        println!("{:?}", details.expires_in());

        println!(
            "Open this URL in your browser:\n{:?}\nand enter the code: {}",
            details.verification_uri(),
            details.user_code().secret(),
        );

        let token = device_client
            .exchange_device_access_token(&details)
            .add_extra_param("code", details.device_code().secret())
            .request_async(&async_http_client, tokio::time::sleep, None)
            .await
            .expect("Failed to get token");

        TraktOAuthToken {
            access_token: token.access_token().secret().to_string(),
            refresh_token: token.refresh_token().unwrap().secret().to_string(),
            expires_in: token.expires_in(),
        }
    }

    pub async fn get_watchlist(&self) -> Result<WatchList, reqwest::Error> {
        self.client
            .request::<WatchList>(
                reqwest::Method::GET,
                "/sync/watchlist/movies,shows",
                None,
                None,
                None,
            )
            .await
    }

    pub async fn search(&mut self, id: u64, kind: &str) -> &Result<SearchResult, reqwest::Error> {
        let search_kind = if kind == "movie" {
            "movie"
        } else if kind == "tv" {
            "show"
        } else {
            "movie"
        };

        let cache_key = format!("{id}-{search_kind}");
        if self.cache.contains_key(&cache_key) {
            log::debug!("Using cached search result for {cache_key}");
            return self.cache.get(&cache_key).unwrap();
        }

        let params = HashMap::from([("type".to_string(), search_kind.to_string())]);
        let result = self
            .client
            .request::<SearchResult>(
                reqwest::Method::GET,
                format!("/search/tmdb/{id}").as_str(),
                Some(params),
                None,
                None,
            )
            .await;

        self.cache.insert(cache_key.clone(), result);
        self.cache.get(&cache_key).unwrap()
    }
}

pub mod client_utils {
    use log;
    use std::str::FromStr;

    use oauth2;
    use thiserror;

    #[derive(Debug, thiserror::Error)]
    pub enum Error<T>
    where
        T: std::error::Error + 'static,
    {
        /// Error returned by reqwest crate.
        #[error("request failed")]
        Reqwest(#[source] T),
        /// Non-reqwest HTTP error.
        #[error("HTTP error")]
        Http(#[source] oauth2::http::Error),
        /// I/O error.
        #[error("I/O error")]
        Io(#[source] std::io::Error),
        /// Other error.
        #[error("Other error: {}", _0)]
        Other(String),
    }

    pub async fn async_http_client(
        request: oauth2::HttpRequest,
    ) -> Result<oauth2::HttpResponse, Error<reqwest::Error>> {
        let client = {
            let builder = reqwest::Client::builder();

            // Following redirects opens the client up to SSRF vulnerabilities.
            // but this is not possible to prevent on wasm targets
            #[cfg(not(target_arch = "wasm32"))]
            let builder = builder.redirect(reqwest::redirect::Policy::none());

            builder.build().map_err(Error::Reqwest)?
        };

        let mut request_builder = client
            .request(reqwest::Method::POST, request.url.as_str())
            .body(request.body.clone());
        for (name, value) in &request.headers {
            request_builder = request_builder.header(name.as_str(), value.as_bytes());
        }
        let request_ = request_builder.build().map_err(Error::Reqwest)?;
        let response = client.execute(request_).await.map_err(Error::Reqwest)?;

        log::debug!(
            "POST {url} - {code} -- {body}",
            url = request.url,
            body = String::from_utf8_lossy(&request.body).to_string(),
            code = response.status(),
        );

        let resp_status_code = u16::from(response.status());
        let status_code = oauth2::http::StatusCode::from_u16(resp_status_code)
            .map_err(|err| Error::Http(err.into()))?;

        let mut headers = oauth2::http::HeaderMap::new();
        let resp_headers = response.headers();
        for (name, value) in resp_headers.iter() {
            let name = name.as_str();
            let header_name = oauth2::http::header::HeaderName::from_str(name).unwrap();
            let value = value.to_str().unwrap().to_owned();
            let header_value = oauth2::http::HeaderValue::from_str(&value).unwrap();
            headers.append(header_name, header_value);
        }

        let chunks = response.bytes().await.map_err(Error::Reqwest)?;
        let body = if chunks.to_vec().is_empty() {
            // This is a utf-8 encoding for {"error": "authorization_pending"}
            vec![
                123, 034, 101, 114, 114, 111, 114, 034, 058, 032, 034, 097, 117, 116, 104, 111,
                114, 105, 122, 097, 116, 105, 111, 110, 095, 112, 101, 110, 100, 105, 110, 103,
                034, 125,
            ]
        } else {
            chunks.to_vec()
        };

        Ok(oauth2::HttpResponse {
            status_code,
            headers,
            body,
        })
    }
}

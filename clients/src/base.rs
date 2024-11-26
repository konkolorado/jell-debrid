use log;
use reqwest;
use serde_json;
use std::collections::HashMap;

pub struct HttpClient {
    base_url: String,
    headers: reqwest::header::HeaderMap,
    pub client: reqwest::Client,
}

impl HttpClient {
    pub fn new(
        base_url: &str,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<Self, reqwest::Error> {
        let mut base_headers = reqwest::header::HeaderMap::new();
        base_headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
        base_headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        if !headers.is_none() {
            base_headers.extend(headers.unwrap());
        }
        Ok(Self {
            client: reqwest::Client::builder().build()?,
            base_url: base_url.to_owned(),
            headers: base_headers,
        })
    }

    pub async fn request<T>(
        &self,
        method: reqwest::Method,
        path: &str,
        params: Option<HashMap<String, String>>,
        json: Option<HashMap<String, String>>,
        data: Option<HashMap<String, String>>,
    ) -> Result<T, reqwest::Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let url_str = format!("{}{}", self.base_url, path);
        let params_ = params.unwrap_or(HashMap::new());
        let url = if params_.is_empty() {
            reqwest::Url::parse(&url_str).unwrap()
        } else {
            reqwest::Url::parse_with_params(&url_str, params_).unwrap()
        };

        let builder: reqwest::RequestBuilder = if data.is_some() {
            let form_data = data.unwrap();
            self.client
                .request(method.clone(), url.clone())
                .headers(self.headers.clone())
                .form(&form_data)
        } else {
            let payload: HashMap<String, String> = json.unwrap_or(HashMap::new());
            self.client
                .request(method.clone(), url.clone())
                .headers(self.headers.clone())
                .json(&payload)
        };

        let response = builder.send().await?;

        log::debug!(
            "{method} {url} - {code}",
            url = url,
            code = response.status(),
        );

        let mut text = response.text().await?;
        if text.is_empty() {
            text.push_str("{}")
        }
        log::debug!("{method} {url} -- {text}", url = url, text = text);
        let resp = serde_json::from_str(&text).unwrap();
        Ok(resp)
    }
}

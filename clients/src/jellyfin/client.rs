use crate::base::HttpClient;
use crate::jellyfin::structs::{NoContent, SystemInfo};

pub struct JellyfinClient {
    client: HttpClient,
}

impl JellyfinClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("MediaBrowser Token={token}").parse().unwrap(),
        );
        Self {
            client: HttpClient::new(base_url, Some(headers)).unwrap(),
        }
    }

    pub async fn get_system_info(&self) -> Result<SystemInfo, reqwest::Error> {
        self.client
            .request::<SystemInfo>(reqwest::Method::GET, "/System/Info", None, None)
            .await
    }

    pub async fn refresh_libraries(&self) -> Result<NoContent, reqwest::Error> {
        self.client
            .request::<NoContent>(reqwest::Method::POST, "/Library/Refresh", None, None)
            .await
    }
}

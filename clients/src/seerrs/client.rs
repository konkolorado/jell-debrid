use std::collections::HashMap;

use crate::base::HttpClient;
use crate::seerrs::structs::Requests;

pub struct SeerrClient {
    client: HttpClient,
}

impl SeerrClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Api-Key", token.parse().unwrap());
        Self {
            client: HttpClient::new(base_url, Some(headers)).unwrap(),
        }
    }

    pub async fn get_unfulfilled_requests(&self) -> Result<Requests, reqwest::Error> {
        let params = HashMap::from([
            ("take".to_string(), "1000".to_string()),
            ("skip".to_string(), "0".to_string()),
            ("filter".to_string(), "processing".to_string()),
        ]);
        self.client
            .request::<Requests>(
                reqwest::Method::GET,
                "/api/v1/request",
                Some(params),
                None,
                None,
            )
            .await
    }
}

use crate::error::{CommandError, CommandResult};
use std::str::FromStr;
use tauri::http::{HeaderMap, HeaderName, HeaderValue};
use tauri_plugin_http::reqwest;

#[derive(Clone)]
pub struct ApiClient {
    pub client: reqwest::Client,
    pub base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }
    pub async fn request(
        &self,
        path: &str,
        method: reqwest::Method,
        payload: Vec<u8>,
        auth_header: HeaderValue,
    ) -> CommandResult<Vec<u8>> {
        let url = format!("{}/{}", self.base_url, path);

        let mut headers = HeaderMap::new();

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_str("application/octet-stream")?,
        );
        headers.insert(reqwest::header::COOKIE, auth_header);

        let res = self
            .client
            .request(method, &url)
            .headers(headers)
            .body(payload)
            .send()
            .await?;

        if res.status().is_success() {
            // Return the updated header map
            return Ok(res.bytes().await?.to_vec());
        }
        Err(CommandError::HttpError(format!("{}", res.status(),)))
    }
}

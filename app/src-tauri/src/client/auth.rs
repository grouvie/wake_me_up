use super::data::client::ApiClient;
use crate::error::{CommandError, CommandResult};
use tauri::http::{HeaderMap, HeaderValue};
use tauri_plugin_http::reqwest;

impl ApiClient {
    pub async fn login(
        &self,
        path: &str,
        method: reqwest::Method,
        payload: Vec<u8>,
    ) -> CommandResult<HeaderValue> {
        let url = format!("{}/{}", self.base_url, path);

        let mut headers = HeaderMap::new();

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_str("application/octet-stream")?,
        );

        let res = self
            .client
            .request(method, &url)
            .headers(headers)
            .body(payload)
            .send()
            .await?;

        if res.status().is_success() {
            if let Some(cookie) = res.cookies().find(|cookie| cookie.name() == "auth-token") {
                let mut header_value = HeaderValue::from_str(
                    format!("{}={}", cookie.name(), cookie.value()).as_str(),
                )?;
                return Ok(header_value);
            } else {
                return Err(CommandError::GenericError(
                    "Authentication token cookie 'auth-token' not found.".to_string(),
                ));
            }
        }
        Err(CommandError::HttpError(format!("{}", res.status(),)))
    }
}

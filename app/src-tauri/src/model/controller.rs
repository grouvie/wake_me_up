use crate::client::data::client::ApiClient;
use tauri::http::HeaderValue;
use tokio::sync::Mutex;

pub struct ModelController {
    pub api_client: ApiClient,
    pub auth_header: Mutex<HeaderValue>,
}

impl ModelController {
    pub async fn new(base_url: &str) -> Self {
        let api_client = ApiClient::new(base_url);
        Self {
            api_client,
            auth_header: Mutex::new(HeaderValue::from_str("").unwrap()),
        }
    }
}

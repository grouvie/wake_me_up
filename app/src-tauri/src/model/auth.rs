use super::controller::ModelController;
use crate::{error::CommandResult, proto::login_request::LoginRequestProto};
use prost::Message;
use tauri_plugin_http::reqwest;

impl ModelController {
    pub async fn login(&self, username: &str, password: &str) -> CommandResult<()> {
        let login_request_proto = LoginRequestProto {
            username: username.to_string(),
            password: password.to_string(),
        };

        let payload = login_request_proto.encode_to_vec();

        let header_value = self
            .api_client
            .login("login", reqwest::Method::POST, payload)
            .await?;

        // Acquire a mutable lock on the auth_header
        let mut auth_header = self.auth_header.lock().await;
        // Update the auth_header value
        *auth_header = header_value;

        Ok(())
    }
}

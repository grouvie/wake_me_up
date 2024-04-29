use std::{env, error::Error, str::FromStr};

use prost::Message;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::proto::login_request::LoginRequestProto;

pub async fn login(url: &str, port: &str) -> Result<HeaderMap, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let username = env::var("CLIENT_USERNAME").expect("CLIENT_USERNAME not supplied.");

    let password = env::var("CLIENT_PASSWORD").expect("CLIENT_PASSWORD not supplied.");

    let login_request = LoginRequestProto { username, password };

    // Serialize the protobuf message into bytes
    let mut buf = Vec::new();
    login_request.encode(&mut buf)?;

    let res = client
        .post(format!("http://{}:{}/api/login", url, port))
        .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
        .body(buf)
        .send()
        .await?;

    if res.status().is_success() {
        if let Some(cookie) = res.cookies().find(|cookie| cookie.name() == "auth-token") {
            let mut header_map = HeaderMap::new();
            header_map.insert(
                HeaderName::from_str("Cookie").unwrap(),
                HeaderValue::from_str(format!("{}={}", cookie.name(), cookie.value()).as_str())?,
            );
            return Ok(header_map);
        } else {
            return Err("Authentication token cookie 'auth-token' not found.".into());
        }
    }

    Err("Request was not successful".into())
}

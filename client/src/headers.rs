use base64::{prelude::BASE64_STANDARD, Engine};
use rand::RngCore;
use reqwest::header::{
    HeaderMap, HeaderValue, CONNECTION, HOST, SEC_WEBSOCKET_EXTENSIONS, SEC_WEBSOCKET_KEY,
    SEC_WEBSOCKET_VERSION, UPGRADE, USER_AGENT,
};

pub fn generate_websocket_headers(host: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    // Set CONNECTION
    let mut key = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut key);
    let sec_websocket_key = BASE64_STANDARD.encode(key);
    headers.insert(
        SEC_WEBSOCKET_KEY,
        HeaderValue::from_str(sec_websocket_key.as_str()).unwrap(),
    );

    // Set CONNECTION
    headers.insert(CONNECTION, HeaderValue::from_static("Upgrade"));

    // Set UPGRADE
    headers.insert(UPGRADE, HeaderValue::from_static("websocket"));

    // Set SEC_WEBSOCKET_VERSION
    headers.insert(SEC_WEBSOCKET_VERSION, HeaderValue::from_static("13"));

    // Set SEC_WEBSOCKET_EXTENSIONS
    headers.insert(
        SEC_WEBSOCKET_EXTENSIONS,
        HeaderValue::from_static("permessage-deflate; client_max_window_bits"),
    );

    // Set HOST
    headers.insert(HOST, HeaderValue::from_str(host).unwrap());

    // Set User Agent
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str("WakeMeUp - Client/1.0").unwrap(),
    );

    headers
}

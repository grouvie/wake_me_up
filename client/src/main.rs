#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::{env, error::Error};

use client::headers::generate_websocket_headers;
use client::login::login;
use client::websocket::connect_websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let host = env::var("HOST").expect("HOST not supplied.");

    let port = env::var("PORT").expect("PORT not supplied.");

    let auth_headers = login(&host, &port).await?;
    let mut headers = generate_websocket_headers(&host);

    headers.extend(auth_headers);

    connect_websocket(&host, &port, headers).await?;

    Ok(())
}

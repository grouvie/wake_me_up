#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::net::SocketAddr;

//#![warn(clippy::cargo)]
use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
    routing::get_service,
    Json, Router,
};
use serde_json::json;
use server::{
    ctx::Ctx, error::MyError, log::log_request, migrations::migrate,
    model::controller::ModelController,
};
use server::{error::MyResult, web};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[tokio::main]
async fn main() -> MyResult<()> {
    dotenv::dotenv().ok();
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher/ equal to INFO (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    // Run database migrations
    migrate().await;

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize ModelController
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_device::routes(mc.clone())
        .merge(web::routes_client::routes(mc.clone()))
        .merge(web::routes_wake_up::routes(mc.clone()))
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_auth::routes(mc.clone()))
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc,
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:    --- Start Server
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Binding TcpListener failed");
    info!(">>> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(
        listener,
        routes_all.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
    // endregion: --- Start Server

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!(">>> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<MyError>();
    let client_status_error = service_error.map(MyError::client_status_and_error);

    // -- If client error, build the new response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            error!("    >>> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error)
        .await
        .unwrap();

    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}

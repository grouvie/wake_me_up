use crate::proto::login_response::LoginResponseProto;
use crate::proto::logout_response::LogoutResponseProto;
use crate::{ctx::Ctx, error::MyResult, web::mw_auth::remove_private_cookie};
use crate::{model::controller::ModelController, web::mw_auth::set_private_cookie};
use crate::{proto::login_request::LoginRequestProto, protobuf::MyProtobuf};
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use axum_extra::protobuf::Protobuf;
use chrono::Utc;
use tower_cookies::Cookies;
use tracing::{debug, info};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/api/login", post(login))
        .route("/api/logout", get(logout))
        .route("/api/validate", get(validate))
        .route("/api/info", get(info))
        .with_state(mc)
}

async fn info(State(mc): State<ModelController>) -> MyResult<String> {
    let clients = mc
        .connected_clients
        .lock()
        .await;
    Ok(format!("{:#?}", clients))
}

async fn login(
    State(mc): State<ModelController>,
    cookies: Cookies,
    MyProtobuf(protobuf): MyProtobuf<LoginRequestProto>,
) -> MyResult<Protobuf<LoginResponseProto>> {
    info!(">>> {:<12} - login", "HANDLER");

    let user_id = mc.check_login(protobuf.0).await?;

    let timestamp = Utc::now().timestamp();
    let token = format!("user-{user_id}.{timestamp}");

    set_private_cookie(&cookies, token);
    // Create the success body.
    let response = LoginResponseProto {
        user_id,
        success: true,
    };

    Ok(Protobuf(response))
}

async fn logout(cookies: Cookies, ctx: Ctx) -> MyResult<Protobuf<LogoutResponseProto>> {
    debug!(">>> {:<12} - logout", "HANDLER");

    remove_private_cookie(&cookies);
    let response = LogoutResponseProto {
        user_id: ctx.user_id(),
        success: true,
    };

    Ok(Protobuf(response))
}

async fn validate(ctx: Ctx) -> MyResult<Protobuf<LoginResponseProto>> {
    debug!(">>> {:<12} - validate", "HANDLER");
    let response = LoginResponseProto {
        user_id: ctx.user_id(),
        success: true,
    };

    Ok(Protobuf(response))
}

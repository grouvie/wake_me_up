use crate::ctx::Ctx;
use crate::error::{MyError, MyResult};
use crate::model::controller::ModelController;
use crate::web::AUTH_TOKEN;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, body::Body};
use chrono::Utc;
use lazy_regex::regex_captures;
use std::{env, sync::LazyLock};
use tower_cookies::cookie::time::Duration;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, Key};
use tracing::debug;

static MY_KEY: LazyLock<String> = LazyLock::new(|| {
    env::var("SECRET_KEY").expect("No SECRET_KEY for cookie encryption provided.")
});

pub async fn mw_require_auth(
    ctx: MyResult<Ctx>,
    req: Request<Body>,
    next: Next,
) -> MyResult<Response> {
    debug!(">>> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> MyResult<Response> {
    debug!(">>> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let key = Key::from(MY_KEY.as_bytes());
    let private_cookies = cookies.private(&key);

    let auth_token = private_cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string());

    // Compute Result<Ctx>.
    let result_ctx = match auth_token
        .ok_or(MyError::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp)) => timestamp_is_valid(&exp).map(|_| {
            let timestamp = Utc::now().timestamp();
            let token = format!("user-{user_id}.{timestamp}");
            set_private_cookie(&cookies, token);
            Ctx::new(user_id)
        }),
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(MyError::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = MyError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> MyResult<Self> {
        debug!(">>> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<MyResult<Ctx>>()
            .ok_or(MyError::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

// endregion: --- Ctx Extractor

// Parse a token of format `user-[user-id].[expiration]`
// Returns (user_id, expiration)
fn parse_token(token: String) -> MyResult<(i32, String)> {
    let (_whole, user_id, exp) = regex_captures!(
        r#"^user-(\d+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(MyError::AuthFailTokenWrongFormat)?;

    let user_id: i32 = user_id
        .parse()
        .map_err(|_| MyError::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string()))
}

pub fn set_private_cookie(cookies: &Cookies, token: String) {
    let key = &Key::from(MY_KEY.as_bytes());

    let private_cookies = cookies.private(key);

    let mut cookie = Cookie::new(AUTH_TOKEN, token);
    cookie.set_path("/");
    // TODO: Set cookie to secure before deployment
    cookie.set_secure(false);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_http_only(true);
    cookie.set_max_age(Duration::hours(1));

    private_cookies.add(cookie);
}

pub fn remove_private_cookie(cookies: &Cookies) {
    let key = &Key::from(MY_KEY.as_bytes());

    let private_cookies = cookies.private(key);
    let mut cookie = Cookie::new(AUTH_TOKEN, "");
    cookie.set_path("/");

    private_cookies.remove(cookie);
}

fn timestamp_is_valid(exp: &str) -> MyResult<()> {
    // Parse the timestamp string as an integer
    let Ok(timestamp) = exp.parse::<i64>() else {
        return Err(MyError::AuthFailInvalidTimestamp);
    };
    let current_timestamp = Utc::now().timestamp();

    let difference = current_timestamp - timestamp;

    // Check if the difference is greater than 1 hour (3600 seconds)
    if difference < 3600 {
        Ok(())
    } else {
        Err(MyError::AuthFailExpiredTokenCookie)
    }
}

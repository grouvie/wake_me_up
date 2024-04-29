use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use strum::AsRefStr;
use tracing::debug;

pub type MyResult<T> = core::result::Result<T, MyError>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum MyError {
    LoginFail,

    // - Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailExpiredTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailInvalidTimestamp,
    AuthFailCtxNotInRequestExt,

    // - Database errors
    PoolCreationFail { error: String },
    ClientCreationFail { error: String },
    Database { error: String },
    DatabaseRowNotFound { error: String },

    // - Protobuf errors
    BytesRejection { error: String },
    ProtobufDecodeError { error: String },

    // - Device errors
    UserDoesNotOwnDevice { user_id: i32, device_id: i32 },

    // - Sender errors
    NoSenderFoundForUser { user_id: i32 },
    FailedToSendWakeUpProto { error: String },

    // - Lock errors
    ConnectedClientsLockFailed,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        debug!(">>> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}

impl MyError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth.
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailInvalidTimestamp
            | Self::AuthFailExpiredTokenCookie
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Database.
            Self::Database { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::DATABASE_ERROR,
            ),
            Self::DatabaseRowNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // - Profobuf.
            Self::BytesRejection { .. } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ClientError::INVALID_PARAMS,
            ),
            Self::ProtobufDecodeError { .. } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ClientError::INVALID_PARAMS,
            ),

            // - Device
            Self::UserDoesNotOwnDevice { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // - Sender
            Self::NoSenderFoundForUser { .. } => {
                (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMS)
            }
            Self::FailedToSendWakeUpProto { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            // - Lock.
            Self::ConnectedClientsLockFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            // - Other.
            Self::ClientCreationFail { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    DATABASE_ERROR,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

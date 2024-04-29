use prost::DecodeError;
use serde::{ser::Serializer, Deserialize, Serialize};
use std::fmt;
use tauri_plugin_http::reqwest;

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    GenericError(String),
    HttpError(String),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    InvalidHeaderValueError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    DecodeError(#[from] DecodeError),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::GenericError(msg) => write!(f, "Generic Error: {}", msg),
            CommandError::HttpError(status_code) => {
                write!(f, "Status Code {}", status_code)
            }
            CommandError::ReqwestError(e) => write!(f, "Reqwest Error: {}", e),
            CommandError::InvalidHeaderValueError(e) => {
                write!(f, "InvalidHeaderValue Error: {}", e)
            }
            CommandError::DecodeError(e) => write!(f, "Decode Error: {}", e),
        }
    }
}

// Implement `Serialize` trait for `CommandError`
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;

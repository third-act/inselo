use std::collections::HashMap;

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unauthenticated")]
    Unauthenticated,

    #[error("Forbidden: you don't have permission to access this resource")]
    Forbidden,

    #[error("Resource not found")]
    NotFound,

    #[error("Validation error: {0:?}")]
    Validation(ValidationError),

    #[error("Request timeout")]
    Timeout,

    #[error("Server error: {status_code}")]
    ServerError { status_code: u16 },

    #[error("Network connection error: {0}")]
    NetworkError(String),

    #[error("Request building error: {0}")]
    RequestError(String),

    #[error("Response parsing error: {message}. Status: {status_code}. Body: {body}")]
    ParseError {
        message: String,
        status_code: StatusCode,
        body: String,
    },

    #[error("Credential provider error: {0}")]
    Credentials(String),

    #[error("Unexpected response code: {0}")]
    UnexpectedResponseCode(StatusCode),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    message: String,
    errors: HashMap<String, Vec<String>>,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_status() {
            if let Some(status) = err.status() {
                match status.as_u16() {
                    401 => return Self::Unauthenticated,
                    403 => return Self::Forbidden,
                    404 => return Self::NotFound,
                    status_code if status_code >= 500 => {
                        return Self::ServerError { status_code };
                    }
                    _ => {}
                }
            }
        }

        if err.is_timeout() {
            return Self::Timeout;
        }

        if err.is_connect() {
            return Self::NetworkError(format!("Connection error: {}", err));
        }

        if err.is_body() {
            return Self::RequestError(format!("Request body error: {}", err));
        }

        if err.is_decode() {
            let status_code = err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            return Self::ParseError {
                message: format!("Failed to decode response: {}", err),
                status_code,
                body: "Response body not captured in From<reqwest::Error> for decode error. Check endpoint logic.".to_string(),
            };
        }

        if err.is_builder() {
            return Self::RequestError(format!("Request builder error: {}", err));
        }

        if err.is_redirect() {
            return Self::NetworkError(format!("Redirect error: {}", err));
        }

        Self::Unknown(err.to_string())
    }
}

use crate::response::ApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use oauth2::basic::BasicErrorResponseType;
use std::{num::ParseIntError, time::SystemTimeError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    SqlError(#[from] sqlx::Error),
    #[error("Request error: {0}")]
    TracingError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Io error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("General error: {0}")]
    GeneralError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Custom error: {0}")]
    CustomError(String),
    #[error("Axum error:{0}")]
    AxumError(#[from] axum::Error),
    #[error("JWT error: {0}")]
    JsonWebError(#[from] jsonwebtoken::errors::Error),
    #[error("Password hashing failed: {0}")]
    HashingFailed(String),
    #[error("TotpUrlError: {0}")]
    TotpUrlError(#[from] totp_rs::TotpUrlError),
    #[error("System time error: {0}")]
    SystemtimeError(#[from] SystemTimeError),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Decode error: {0}")]
    DecodeError(#[from] base64::DecodeError),
    #[error("SMTP error: {0}")]
    SMTPError(#[from] lettre::transport::smtp::Error),
    #[error("Address error: {0}")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("UrlParse error: {0}")]
    ParseError(#[from] url::ParseError),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serde Json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Multipart error: {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("Object error: {0}")]
    ObjectError(#[from] object_store::Error),
    #[error("Uuid error: {0}")]
    UuidError(#[from] sqlx::types::uuid::Error),
    #[error("ParseInt: {0}")]
    ParseIntError(#[from] ParseIntError),
    // #[error("Ngrok connect error: {0}")]
    // ConnectError(#[from] ngrok::session::ConnectError),
    // #[error("Ngrok rpc error: {0}")]
    // RpcError(#[from] ngrok::session::RpcError),
    #[error("Token Request error: {0}")]
    TokenRequestError(
        #[from]
        oauth2::RequestTokenError<
            oauth2::HttpClientError<reqwest::Error>,
            oauth2::StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::CustomError(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::CustomError(s.to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Error::HashingFailed(err.to_string())
    }
}

impl From<chacha20poly1305::Error> for Error {
    fn from(err: chacha20poly1305::Error) -> Self {
        Error::EncryptionError(err.to_string())
    }
}

impl Into<ApiResponse<String>> for Error {
    fn into(self) -> ApiResponse<String> {
        let (status, message) = match self {
            Error::SqlError(ref err) => {
                // Check for specific constraint violations
                if let sqlx::Error::Database(db_err) = err {
                    if db_err.constraint() == Some("users_email_key") {
                        (StatusCode::CONFLICT, "Email already registered".to_string())
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Database error".to_string(),
                        )
                    }
                } else {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
                    )
                }
            }
            Error::TracingError(ref msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            Error::IoError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::EnvVarError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::GeneralError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::CustomError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::AxumError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::JsonWebError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::HashingFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::TotpUrlError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::SystemtimeError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::ObjectError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::EncryptionError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::Utf8Error(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::DecodeError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::SMTPError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::AddressError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::ParseError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::RedisError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::SerdeJsonError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::MultipartError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::ParseIntError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::UuidError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::RequestError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::TokenRequestError(ref msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string())
            }
        };

        ApiResponse {
            // status: "failed".to_string(),
            message,
            code: status.as_u16(),
            data: "".to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let response: ApiResponse<String> = self.into();
        response.into_response()
    }
}

#[macro_export]
macro_rules! error {
    ($variant:ident, $($arg:tt)*) => {
        Error::$variant(format!($($arg)*))
    };
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_macro() {
        let error = error!(CustomError, "Testing 123 : {}", 69);
        let x_error = Error::CustomError(format!("Testing 123 : {}", 69));

        // Test
        assert_eq!(error.to_string(), x_error.to_string());
    }
}

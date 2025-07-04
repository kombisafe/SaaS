use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use duration_str::DError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Password error: {0}")]
    Password(#[from] crate::auth::password::PasswordError),

    #[error("UUID parsing error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Duration parsing error: {0}")]
    ParseDuration(#[from] DError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Sqlx(err) => {
                tracing::error!("SQLx error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AppError::Jwt(err) => {
                tracing::error!("JWT error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "JWT processing error".to_string(),
                )
            }
            AppError::Redis(err) => {
                tracing::error!("Redis error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Cache error".to_string(),
                )
            }
            AppError::Config(err) => {
                tracing::error!("Config error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Configuration error".to_string(),
                )
            }
            AppError::Password(err) => {
                tracing::error!("Password error: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "Password error".to_string(),
                )
            }
            AppError::Uuid(err) => {
                tracing::error!("UUID parsing error: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "UUID parsing error".to_string(),
                )
            }
            AppError::ParseDuration(err) => {
                tracing::error!("Duration parsing error: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "Duration parsing error".to_string(),
                )
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

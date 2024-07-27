// src/errors/mod.rs
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub(crate) status: String,
    pub(crate) message: T,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Password hashing error")]
    PasswordHashingError,
    #[error("Invalid request data: {0}")]
    InvalidRequestData(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            ApiError::ValidationError(detail) => (
                "error".to_string(),
                format!("Validation failed: {}", detail),
            ),
            ApiError::DatabaseError(detail) => (
                "error".to_string(),
                format!("Failed to register user: {}", detail),
            ),
            ApiError::PasswordHashingError => {
                ("error".to_string(), "Password hashing failed".to_string())
            }
            ApiError::InvalidRequestData(detail) => (
                "error".to_string(),
                format!("Invalid request data: {}", detail),
            ),
            ApiError::InternalServerError => {
                ("error".to_string(), "Internal server error".to_string())
            }
        };

        let response = ApiResponse { status, message };

        HttpResponse::InternalServerError().json(response)
    }
}

//! # HATEOAS Errors
//!
//! Domain-specific errors for HATEOAS link generation and resource construction.

use axum::http::StatusCode;
use eywa_errors::AppError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HateoasError {
    #[error("Failed to generate URL: {0}")]
    UrlGenerationError(String),

    #[error("Invalid resource state: {0}")]
    InvalidResource(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<HateoasError> for AppError {
    fn from(err: HateoasError) -> Self {
        match err {
            // URL generation failure is usually an internal configuration or logic error
            HateoasError::UrlGenerationError(msg) => {
                AppError::InternalServerError(format!("HATEOAS URL Error: {}", msg))
            }

            // Invalid resource state might be a data consistency issue
            HateoasError::InvalidResource(msg) => {
                AppError::InternalServerError(format!("HATEOAS Resource Error: {}", msg))
            }

            // Serialization issues are internal
            HateoasError::SerializationError(msg) => {
                AppError::InternalServerError(format!("HATEOAS Serialization Error: {}", msg))
            }
        }
    }
}

impl HateoasError {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

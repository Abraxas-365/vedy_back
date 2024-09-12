use actix_web::{HttpResponse, ResponseError};
use serde_json::Error as SerdeError;
use sqlx::Error as SqlxError;
use thiserror::Error;

use crate::utils::lucia;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] SerdeError),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Lucia error: {0}")]
    LuciaError(#[from] lucia::Error),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ParseError(_) => HttpResponse::InternalServerError().body(self.to_string()),
            ApiError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            ApiError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            ApiError::NotFound(_) => HttpResponse::NotFound().json(self.to_string()),
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(self.to_string()),
            ApiError::Forbidden(_) => HttpResponse::Forbidden().json(self.to_string()),
            ApiError::Unauthorized(_) => HttpResponse::Unauthorized().json(self.to_string()),
            ApiError::Conflict(_) => HttpResponse::Conflict().json(self.to_string()),
            ApiError::ServiceUnavailable(_) => {
                HttpResponse::ServiceUnavailable().json(self.to_string())
            }
            ApiError::LuciaError(lucia_error) => match lucia_error {
                lucia::Error::DatabaseConnectionError(_) => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::UserSessionNotFound => {
                    HttpResponse::NotFound().json(self.to_string())
                }
                lucia::Error::InvalidSessionId => HttpResponse::BadRequest().json(self.to_string()),
                lucia::Error::SessionExpired => HttpResponse::Unauthorized().json(self.to_string()),
                lucia::Error::DatabaseQueryError(_) => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::UserSessionTableNotExist => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::AuthUserTableNotExist => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::InvalidCredentials => {
                    HttpResponse::Unauthorized().json(self.to_string())
                }
                lucia::Error::DuplicateUserError(_) => {
                    HttpResponse::Conflict().json(self.to_string())
                }
                lucia::Error::SessionCreationFailed => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::SessionDeletionFailed => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::UserCreationFailed => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::UserUpdateFailed => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::EncryptionError(_) => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::DecryptionError(_) => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::InvalidToken => HttpResponse::Unauthorized().json(self.to_string()),
                lucia::Error::TokenExpired => HttpResponse::Unauthorized().json(self.to_string()),
                lucia::Error::ConfigurationError(_) => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
                lucia::Error::UnexpectedError(_) => {
                    HttpResponse::InternalServerError().json(self.to_string())
                }
            },
        }
    }
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Server error: {0}")]
    Server(#[from] std::io::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("{0}")]
    Other(String),
}

#[derive(Serialize)]
struct ErrorResponse { error: String }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[cfg(debug_assertions)]
        tracing::error!(error = ?self, "request failed");
        #[cfg(not(debug_assertions))]
        tracing::warn!(error = %self, "request failed");

        let (status, msg) = match &self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Server(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        };

        let body = Json(ErrorResponse { error: msg.to_string() });
        (status, body).into_response()
    }
}

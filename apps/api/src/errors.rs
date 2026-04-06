use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("validation: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal")]
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!(error = %self, "internal error");
        }

        (status, Json(ErrorBody { error: message })).into_response()
    }
}

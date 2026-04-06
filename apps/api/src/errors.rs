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

    #[error("rate limited")]
    RateLimited,
}

impl ApiError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "not_found",
            Self::Unauthorized => "unauthorized",
            Self::Validation(_) => "validation",
            Self::Conflict(_) => "conflict",
            Self::Internal => "internal",
            Self::RateLimited => "rate_limited",
        }
    }
}

/// JSON error body aligned with `packages/types` `ApiErrorBody`.
#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
    pub code: &'static str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::RateLimited => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
        };

        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!(error = %self, "internal error");
        }

        let body = ErrorBody {
            error: message,
            code: self.code(),
        };
        (status, Json(body)).into_response()
    }
}

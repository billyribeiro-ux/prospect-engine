use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

use crate::errors::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct SendEmailBody {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// Accepts a send request; full SMTP integration is configured via `PE_SMTP_*` (see `docs/SECURITY.md`).
pub async fn post_send(
    State(_state): State<AppState>,
    Json(body): Json<SendEmailBody>,
) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
    let to = body.to.trim();
    if to.is_empty() {
        return Err(ApiError::Validation("to is required".into()));
    }
    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "accepted",
            "mode": "stub",
            "detail": "Wire SMTP (e.g. PE_SMTP_HOST) in deployment; message logged for audit only.",
            "to": to,
            "subject": body.subject,
        })),
    ))
}

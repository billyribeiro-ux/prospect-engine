use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::AppState;

/// Minimal 1×1 transparent GIF (43 bytes).
static OPEN_PIXEL_GIF: &[u8] = &[
    0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x00, 0x21, 0xf9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x44, 0x01, 0x00, 0x3b,
];

#[derive(Deserialize)]
pub struct ClickQuery {
    pub u: String,
}

/// Records an open and returns a tracking pixel.
pub async fn get_open_pixel(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let t = token.trim();
    if t.is_empty() {
        return Err(ApiError::Validation("token required".into()));
    }
    sqlx::query("UPDATE email_events SET opens = opens + 1 WHERE tracking_token = ?")
        .bind(t)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "email open track");
            ApiError::Internal
        })?;
    Ok((
        [(header::CONTENT_TYPE, "image/gif")],
        Body::from(OPEN_PIXEL_GIF),
    ))
}

/// Records a click and redirects to `u` (must be http/https).
pub async fn get_click_redirect(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Query(q): Query<ClickQuery>,
) -> Result<Redirect, ApiError> {
    let t = token.trim();
    if t.is_empty() {
        return Err(ApiError::Validation("token required".into()));
    }
    let target = q.u.trim();
    let parsed =
        url::Url::parse(target).map_err(|_| ApiError::Validation("invalid u URL".into()))?;
    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return Err(ApiError::Validation("u must be http or https".into()));
    }

    sqlx::query("UPDATE email_events SET clicks = clicks + 1 WHERE tracking_token = ?")
        .bind(t)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "email click track");
            ApiError::Internal
        })?;

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO email_click_log (id, tracking_token, target_url, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(t)
    .bind(target)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "email_click_log");
        ApiError::Internal
    })?;

    Ok(Redirect::temporary(target))
}

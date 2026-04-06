use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::errors::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct DiscoveryQuery {
    pub url: Option<String>,
}

pub async fn get_discovery(
    State(_state): State<AppState>,
    Query(q): Query<DiscoveryQuery>,
) -> Result<Json<Value>, ApiError> {
    if let Some(url) = q.url.as_ref().map(|s| s.trim().to_string()) {
        if !url.is_empty() {
            let body = crawler::fetch_url(&url)
                .await
                .map_err(|e| ApiError::Validation(format!("fetch failed: {e}")))?;
            let preview: String = body.chars().take(2000).collect();
            return Ok(Json(json!({
                "status": "fetched",
                "url": url,
                "preview_chars": preview.len(),
                "preview": preview,
            })));
        }
    }
    Ok(Json(
        json!({ "status": "stub", "sources": [], "hint": "pass ?url=https://… to fetch HTML" }),
    ))
}

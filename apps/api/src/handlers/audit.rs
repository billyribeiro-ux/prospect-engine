use axum::extract::Query;
use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct AuditQuery {
    pub url: Option<String>,
}

#[derive(Deserialize)]
pub struct RunAuditBody {
    pub html: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuditResponse {
    status: &'static str,
    #[serde(flatten)]
    score: scorer::AuditScore,
    graded_at: String,
}

fn score_html_or_error(html: &str) -> Result<scorer::AuditScore, ApiError> {
    if html.trim().is_empty() {
        return Err(ApiError::Validation("html content is empty".into()));
    }
    Ok(scorer::score_html(html))
}

/// `GET /audit` — optional `?url=` fetches HTML then scores; without `url`, returns usage hints.
pub async fn get_audit(Query(q): Query<AuditQuery>) -> Result<Json<Value>, ApiError> {
    let Some(raw) = q.url else {
        return Ok(Json(json!({
            "status": "hint",
            "message": "Provide ?url=https://… to run a heuristic audit, or POST /api/v1/audit/run with { \"html\" } or { \"url\" }.",
        })));
    };
    let url = raw.trim();
    if url.is_empty() {
        return Err(ApiError::Validation("url must not be empty".into()));
    }
    let html = crawler::fetch_url(url)
        .await
        .map_err(|e| ApiError::Validation(format!("fetch failed: {e}")))?;
    let score = score_html_or_error(&html)?;
    let body = AuditResponse {
        status: "complete",
        score,
        graded_at: Utc::now().to_rfc3339(),
    };
    Ok(Json(
        serde_json::to_value(body).map_err(|_| ApiError::Internal)?,
    ))
}

/// `POST /audit/run` — score raw HTML or fetch by URL first.
pub async fn post_audit_run(Json(body): Json<RunAuditBody>) -> Result<Json<Value>, ApiError> {
    let html = if let Some(h) = body.html.filter(|s| !s.trim().is_empty()) {
        h
    } else if let Some(u) = body
        .url
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        crawler::fetch_url(u)
            .await
            .map_err(|e| ApiError::Validation(format!("fetch failed: {e}")))?
    } else {
        return Err(ApiError::Validation(
            "provide \"html\" and/or \"url\" (at least one)".into(),
        ));
    };
    let score = score_html_or_error(&html)?;
    let res = AuditResponse {
        status: "complete",
        score,
        graded_at: Utc::now().to_rfc3339(),
    };
    Ok(Json(
        serde_json::to_value(res).map_err(|_| ApiError::Internal)?,
    ))
}

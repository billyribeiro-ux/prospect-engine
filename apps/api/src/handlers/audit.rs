use axum::extract::Query;
use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};

use crate::errors::ApiError;
use crate::services::headless;

#[derive(Deserialize)]
pub struct AuditQuery {
    pub url: Option<String>,
    /// When `true`, try `PE_CHROME_BIN` headless `--dump-dom` before falling back to HTTP fetch.
    pub headless: Option<bool>,
}

#[derive(Deserialize)]
pub struct RunAuditBody {
    pub html: Option<String>,
    pub url: Option<String>,
    pub headless: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuditResponse {
    status: &'static str,
    #[serde(flatten)]
    score: scorer::AuditScore,
    graded_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    headless: Option<String>,
}

fn score_html_or_error(html: &str) -> Result<scorer::AuditScore, ApiError> {
    if html.trim().is_empty() {
        return Err(ApiError::Validation("html content is empty".into()));
    }
    Ok(scorer::score_html(html))
}

async fn fetch_html_audit(
    url: &str,
    prefer_headless: bool,
) -> Result<(String, Option<String>), ApiError> {
    if prefer_headless {
        match headless::dump_dom(url).await {
            Ok(h) => return Ok((h, Some("headless_chrome".to_string()))),
            Err(e) => {
                tracing::warn!(error = %e, "headless audit failed; falling back to HTTP fetch");
            }
        }
    }
    let h = crawler::fetch_url(url)
        .await
        .map_err(|e| ApiError::Validation(format!("fetch failed: {e}")))?;
    Ok((h, None))
}

/// `GET /audit` — optional `?url=` fetches HTML then scores; `headless=1` tries Chromium first.
pub async fn get_audit(Query(q): Query<AuditQuery>) -> Result<Json<Value>, ApiError> {
    let Some(raw) = q.url else {
        return Ok(Json(json!({
            "status": "hint",
            "message": "Provide ?url=https://… to run a heuristic audit, or POST /api/v1/audit/run with { \"html\" } or { \"url\" }. Optional headless=1 uses PE_CHROME_BIN.",
        })));
    };
    let url = raw.trim();
    if url.is_empty() {
        return Err(ApiError::Validation("url must not be empty".into()));
    }
    let prefer_headless = q.headless == Some(true);
    let (html, hl) = fetch_html_audit(url, prefer_headless).await?;
    let score = score_html_or_error(&html)?;
    let body = AuditResponse {
        status: "complete",
        score,
        graded_at: Utc::now().to_rfc3339(),
        headless: hl,
    };
    Ok(Json(
        serde_json::to_value(body).map_err(|_| ApiError::Internal)?,
    ))
}

/// `POST /audit/run` — score raw HTML or fetch by URL first.
pub async fn post_audit_run(Json(body): Json<RunAuditBody>) -> Result<Json<Value>, ApiError> {
    let prefer_headless = body.headless == Some(true);
    let (html, hl) = if let Some(h) = body.html.filter(|s| !s.trim().is_empty()) {
        (h, None)
    } else if let Some(u) = body
        .url
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        fetch_html_audit(u, prefer_headless).await?
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
        headless: hl,
    };
    Ok(Json(
        serde_json::to_value(res).map_err(|_| ApiError::Internal)?,
    ))
}

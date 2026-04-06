use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use queue::JobQueue;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::errors::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct DiscoveryQuery {
    pub url: Option<String>,
}

#[derive(Deserialize)]
pub struct DiscoveryJobBody {
    pub url: String,
}

/// `GET /discovery` — optional `?url=` fetches HTML preview; otherwise describes the endpoint.
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
    Ok(Json(json!({
        "status": "ready",
        "sources": [],
        "hint": "pass ?url=https://… to fetch HTML, or POST /api/v1/discovery/jobs to enqueue a crawl job",
    })))
}

/// `POST /discovery/jobs` — enqueue a durable discovery job (background worker fetches URL).
pub async fn post_discovery_job(
    State(state): State<AppState>,
    Json(body): Json<DiscoveryJobBody>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let url = body.url.trim();
    if url.is_empty() {
        return Err(ApiError::Validation("url is required".into()));
    }
    let mem_id = Uuid::new_v4();
    let job_id = format!("discovery:{mem_id}:{url}");
    state.job_queue.enqueue(&job_id).await.map_err(|e| {
        tracing::error!(error = %e, "enqueue discovery job");
        ApiError::Internal
    })?;

    let durable_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let payload = json!({ "url": url }).to_string();
    sqlx::query(
        "INSERT INTO durable_jobs (id, kind, payload, status, created_at, updated_at) \
         VALUES (?, 'discovery', ?, 'pending', ?, ?)",
    )
    .bind(&durable_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "durable discovery job");
        ApiError::Internal
    })?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "queued",
            "job_id": job_id,
            "durable_job_id": durable_id,
            "url": url,
        })),
    ))
}

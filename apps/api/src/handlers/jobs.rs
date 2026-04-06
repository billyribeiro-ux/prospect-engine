//! Job queue endpoints — in-memory FIFO plus durable `durable_jobs` with background worker.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use queue::JobQueue;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::errors::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct SubmitJobBody {
    pub job_id: String,
}

#[derive(Serialize)]
pub struct SubmitJobResponse {
    pub status: &'static str,
    pub job_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_job_id: Option<String>,
}

#[derive(Serialize)]
pub struct QueueStatsResponse {
    pub depth: usize,
    pub durable_pending: i64,
}

pub async fn post_job(
    State(state): State<AppState>,
    Json(body): Json<SubmitJobBody>,
) -> Result<(StatusCode, Json<SubmitJobResponse>), ApiError> {
    let job_id = body.job_id.trim();
    if job_id.is_empty() {
        return Err(ApiError::Validation("job_id is required".into()));
    }
    state.job_queue.enqueue(job_id).await.map_err(|e| {
        tracing::error!(error = %e, "enqueue job");
        ApiError::Internal
    })?;

    let durable_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let payload = json!({ "job_id": job_id }).to_string();
    sqlx::query(
        "INSERT INTO durable_jobs (id, kind, payload, status, created_at, updated_at) \
         VALUES (?, 'generic', ?, 'pending', ?, ?)",
    )
    .bind(&durable_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "durable job insert");
        ApiError::Internal
    })?;

    Ok((
        StatusCode::ACCEPTED,
        Json(SubmitJobResponse {
            status: "queued",
            job_id: job_id.to_string(),
            durable_job_id: Some(durable_id),
        }),
    ))
}

pub async fn get_queue_stats(
    State(state): State<AppState>,
) -> Result<Json<QueueStatsResponse>, ApiError> {
    let depth = state.job_queue.depth().await;
    let row: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM durable_jobs WHERE status IN ('pending', 'running')")
            .fetch_one(&state.pool)
            .await
            .map_err(|_| ApiError::Internal)?;
    Ok(Json(QueueStatsResponse {
        depth,
        durable_pending: row.0,
    }))
}

type DurableJobRow = (
    String,
    String,
    String,
    String,
    Option<String>,
    Option<String>,
    String,
    String,
);

/// `GET /jobs/durable/{id}` — inspect durable job status.
pub async fn get_durable_job(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let row: Option<DurableJobRow> = sqlx::query_as(
        "SELECT id, kind, payload, status, error, result_summary, created_at, updated_at \
         FROM durable_jobs WHERE id = ?",
    )
    .bind(&job_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;
    let Some((id, kind, payload, status, error, result_summary, created_at, updated_at)) = row
    else {
        return Err(ApiError::NotFound(format!(
            "durable job not found: {job_id}"
        )));
    };
    Ok(Json(json!({
        "id": id,
        "kind": kind,
        "payload": serde_json::from_str::<Value>(&payload).unwrap_or(json!(payload)),
        "status": status,
        "error": error,
        "result_summary": result_summary,
        "created_at": created_at,
        "updated_at": updated_at,
    })))
}

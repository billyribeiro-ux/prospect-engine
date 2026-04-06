//! Job queue endpoints — `MemoryQueue` wired for local/dev; swap implementation later.

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use queue::JobQueue;
use serde::{Deserialize, Serialize};

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
}

#[derive(Serialize)]
pub struct QueueStatsResponse {
    pub depth: usize,
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
    Ok((
        StatusCode::ACCEPTED,
        Json(SubmitJobResponse {
            status: "queued",
            job_id: job_id.to_string(),
        }),
    ))
}

pub async fn get_queue_stats(State(state): State<AppState>) -> Json<QueueStatsResponse> {
    let depth = state.job_queue.depth().await;
    Json(QueueStatsResponse { depth })
}

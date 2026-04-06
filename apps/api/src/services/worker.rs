//! Background worker: claims `durable_jobs` rows and executes discovery crawls.

use std::time::Duration;

use serde_json::Value;
use sqlx::AnyPool;
use tokio::time::sleep;

/// Runs until the process exits. Single-threaded claim loop (SQLite-safe).
pub async fn run_durable_worker(pool: AnyPool) {
    loop {
        if let Err(e) = tick(&pool).await {
            tracing::warn!(error = %e, "durable worker tick");
        }
        sleep(Duration::from_secs(2)).await;
    }
}

async fn tick(pool: &AnyPool) -> Result<(), sqlx::Error> {
    let row: Option<(String, String, String)> = sqlx::query_as(
        "SELECT id, kind, payload FROM durable_jobs WHERE status = 'pending' ORDER BY created_at LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    let Some((id, kind, payload)) = row else {
        return Ok(());
    };

    let now = chrono::Utc::now().to_rfc3339();
    let n = sqlx::query(
        "UPDATE durable_jobs SET status = 'running', updated_at = ? WHERE id = ? AND status = 'pending'",
    )
    .bind(&now)
    .bind(&id)
    .execute(pool)
    .await?
    .rows_affected();

    if n == 0 {
        return Ok(());
    }

    let result = run_kind(&kind, &payload).await;
    let done = chrono::Utc::now().to_rfc3339();
    match result {
        Ok(summary) => {
            sqlx::query(
                "UPDATE durable_jobs SET status = 'completed', result_summary = ?, error = NULL, updated_at = ? WHERE id = ?",
            )
            .bind(&summary)
            .bind(&done)
            .bind(&id)
            .execute(pool)
            .await?;
        }
        Err(err) => {
            tracing::warn!(job_id = %id, error = %err, "durable job failed");
            sqlx::query(
                "UPDATE durable_jobs SET status = 'failed', error = ?, updated_at = ? WHERE id = ?",
            )
            .bind(&err)
            .bind(&done)
            .bind(&id)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

async fn run_kind(kind: &str, payload: &str) -> Result<String, String> {
    match kind {
        "discovery" => {
            let v: Value = serde_json::from_str(payload).map_err(|e| e.to_string())?;
            let url = v
                .get("url")
                .and_then(|x| x.as_str())
                .ok_or_else(|| "payload.url missing".to_string())?;
            let body = crawler::fetch_url(url).await.map_err(|e| e.to_string())?;
            Ok(format!("fetched {} bytes", body.len()))
        }
        "generic" => Ok("ack".to_string()),
        _ => Err(format!("unknown job kind: {kind}")),
    }
}

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::errors::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateLeadBody {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct LeadRow {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub status: String,
    pub created_at: String,
}

pub async fn get_leads(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let rows: Vec<(String, String, Option<String>, String, String)> = sqlx::query_as(
        "SELECT id, name, email, status, created_at FROM leads ORDER BY created_at DESC LIMIT 200",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "list leads");
        ApiError::Internal
    })?;
    let leads: Vec<LeadRow> = rows
        .into_iter()
        .map(|(id, name, email, status, created_at)| LeadRow {
            id,
            name,
            email,
            status,
            created_at,
        })
        .collect();
    Ok(Json(json!({ "status": "ok", "leads": leads })))
}

pub async fn post_lead(
    State(state): State<AppState>,
    Json(body): Json<CreateLeadBody>,
) -> Result<Json<Value>, ApiError> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::Validation("name is required".into()));
    }
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO leads (id, name, email, status, created_at) VALUES (?, ?, ?, 'new', ?)",
    )
    .bind(&id)
    .bind(name)
    .bind(&body.email)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "insert lead");
        ApiError::Internal
    })?;
    Ok(Json(json!({
        "status": "created",
        "id": id,
    })))
}

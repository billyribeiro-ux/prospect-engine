use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateLeadBody {
    pub name: String,
    pub email: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(FromRow)]
struct LeadListRow {
    id: String,
    name: String,
    email: Option<String>,
    status: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
    created_at: String,
}

#[derive(Serialize)]
pub struct LeadRow {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub status: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: String,
}

pub async fn get_leads(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let rows: Vec<LeadListRow> = sqlx::query_as(
        "SELECT id, name, email, status, latitude, longitude, created_at \
         FROM leads ORDER BY created_at DESC LIMIT 200",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "list leads");
        ApiError::Internal
    })?;
    let leads: Vec<LeadRow> = rows
        .into_iter()
        .map(|r| LeadRow {
            id: r.id,
            name: r.name,
            email: r.email,
            status: r.status,
            latitude: r.latitude,
            longitude: r.longitude,
            created_at: r.created_at,
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
        "INSERT INTO leads (id, name, email, status, created_at, latitude, longitude) \
         VALUES (?, ?, ?, 'new', ?, ?, ?)",
    )
    .bind(&id)
    .bind(name)
    .bind(&body.email)
    .bind(&now)
    .bind(body.latitude)
    .bind(body.longitude)
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

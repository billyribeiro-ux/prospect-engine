use axum::extract::State;
use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};

use crate::errors::ApiError;
use crate::AppState;

#[derive(Serialize)]
struct MapMarker {
    id: String,
    label: String,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<f64>,
}

/// Returns CRM leads that have coordinates (for `MapLibre` markers).
pub async fn get_map(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let rows: Vec<(String, String, f64, f64)> = sqlx::query_as(
        "SELECT id, name, latitude, longitude FROM leads \
         WHERE latitude IS NOT NULL AND longitude IS NOT NULL \
         ORDER BY created_at DESC LIMIT 500",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "map markers");
        ApiError::Internal
    })?;
    let markers: Vec<MapMarker> = rows
        .into_iter()
        .map(|(id, name, lat, lng)| MapMarker {
            id,
            label: name,
            latitude: lat,
            longitude: lng,
            score: None,
        })
        .collect();
    Ok(Json(json!({
        "status": "ok",
        "markers": markers,
    })))
}

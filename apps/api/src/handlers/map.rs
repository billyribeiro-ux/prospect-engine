use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
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

#[derive(Serialize)]
struct HeatmapPoint {
    latitude: f64,
    longitude: f64,
    weight: f64,
}

#[derive(Deserialize)]
pub struct MapRouteQuery {
    pub from_id: String,
    pub to_id: String,
}

/// Haversine distance on the WGS84 sphere (meters).
fn haversine_meters(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371_000.0;
    let p1 = lat1.to_radians();
    let p2 = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let h = (dlat / 2.0).sin().powi(2) + p1.cos() * p2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * h.sqrt().asin();
    R * c
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

/// Weighted points for heatmap layers (same lead set; weight = 1.0 each).
pub async fn get_map_heatmap(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let rows: Vec<(f64, f64)> = sqlx::query_as(
        "SELECT latitude, longitude FROM leads \
         WHERE latitude IS NOT NULL AND longitude IS NOT NULL \
         ORDER BY created_at DESC LIMIT 2000",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "map heatmap");
        ApiError::Internal
    })?;
    let points: Vec<HeatmapPoint> = rows
        .into_iter()
        .map(|(latitude, longitude)| HeatmapPoint {
            latitude,
            longitude,
            weight: 1.0,
        })
        .collect();
    Ok(Json(json!({
        "status": "ok",
        "points": points,
    })))
}

/// Great-circle route between two leads as a `GeoJSON` `LineString` plus distance in meters.
pub async fn get_map_route(
    State(state): State<AppState>,
    Query(q): Query<MapRouteQuery>,
) -> Result<Json<Value>, ApiError> {
    let a: Option<(f64, f64)> = sqlx::query_as(
        "SELECT latitude, longitude FROM leads WHERE id = ? AND latitude IS NOT NULL AND longitude IS NOT NULL",
    )
    .bind(&q.from_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;
    let b: Option<(f64, f64)> = sqlx::query_as(
        "SELECT latitude, longitude FROM leads WHERE id = ? AND latitude IS NOT NULL AND longitude IS NOT NULL",
    )
    .bind(&q.to_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;
    let (Some((lat1, lon1)), Some((lat2, lon2))) = (a, b) else {
        return Err(ApiError::Validation(
            "from_id and to_id must reference leads with coordinates".into(),
        ));
    };
    let distance_meters = haversine_meters(lat1, lon1, lat2, lon2);
    Ok(Json(json!({
        "status": "ok",
        "distance_meters": distance_meters,
        "geojson": {
            "type": "LineString",
            "coordinates": [[lon1, lat1], [lon2, lat2]],
        },
    })))
}

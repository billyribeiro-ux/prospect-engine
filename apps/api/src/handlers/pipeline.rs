use axum::Json;
use serde_json::{json, Value};

pub async fn get_pipeline() -> Json<Value> {
	Json(json!({ "status": "stub", "stages": [] }))
}

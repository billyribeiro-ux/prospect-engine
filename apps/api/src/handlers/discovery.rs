use axum::Json;
use serde_json::{json, Value};

pub async fn get_discovery() -> Json<Value> {
	Json(json!({ "status": "stub", "sources": [] }))
}

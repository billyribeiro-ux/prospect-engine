use axum::Json;
use serde_json::{json, Value};

pub async fn get_audit() -> Json<Value> {
	Json(json!({ "status": "stub", "dimensions": [] }))
}

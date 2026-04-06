use axum::Json;
use serde_json::{json, Value};

pub async fn get_map() -> Json<Value> {
	Json(json!({ "status": "stub", "bounds": null }))
}

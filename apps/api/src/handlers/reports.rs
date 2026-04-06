use axum::Json;
use serde_json::{json, Value};

pub async fn get_reports() -> Json<Value> {
	Json(json!({ "status": "stub", "reports": [] }))
}

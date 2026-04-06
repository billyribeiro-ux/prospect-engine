use axum::Json;
use serde_json::{json, Value};

pub async fn get_session() -> Json<Value> {
	Json(json!({ "authenticated": false }))
}

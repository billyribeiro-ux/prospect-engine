use axum::extract::State;
use axum::http::Uri;
use axum::routing::get;
use axum::Router;

use crate::errors::ApiError;
use crate::handlers;
use crate::AppState;

pub fn api_router(state: AppState) -> Router {
	Router::new()
		.route("/health", get(handlers::health::get_health))
		.route("/api/v1/health", get(handlers::health::get_health))
		.fallback(fallback)
		.with_state(state)
}

async fn fallback(State(_state): State<AppState>, uri: Uri) -> ApiError {
	ApiError::NotFound(uri.to_string())
}

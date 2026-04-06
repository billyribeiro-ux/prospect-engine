use axum::extract::State;
use axum::http::Uri;
use axum::routing::get;
use axum::Router;

use crate::errors::ApiError;
use crate::handlers;
use crate::AppState;

pub fn api_router(state: AppState) -> Router {
	let v1 = Router::new()
		.route("/health", get(handlers::health::get_health))
		.route("/auth/session", get(handlers::auth::get_session))
		.route("/discovery", get(handlers::discovery::get_discovery))
		.route("/audit", get(handlers::audit::get_audit))
		.route("/pipeline", get(handlers::pipeline::get_pipeline))
		.route("/reports", get(handlers::reports::get_reports))
		.route("/map", get(handlers::map::get_map))
		.route("/ws", get(handlers::ws::ws_stub));

	Router::new()
		.route("/health", get(handlers::health::get_health))
		.nest("/api/v1", v1)
		.fallback(fallback)
		.with_state(state)
}

async fn fallback(State(_state): State<AppState>, uri: Uri) -> ApiError {
	ApiError::NotFound(uri.to_string())
}

use axum::extract::State;
use axum::http::Uri;
use axum::routing::{get, post};
use axum::Router;

use crate::errors::ApiError;
use crate::handlers;
use crate::state::AppState;

pub fn api_router(state: AppState) -> Router {
    let v1 = Router::new()
        .route("/health", get(handlers::health::get_health))
        .route("/auth/session", get(handlers::auth::get_session))
        .route("/auth/register", post(handlers::auth::post_register))
        .route("/auth/login", post(handlers::auth::post_login))
        .route("/auth/refresh", post(handlers::auth::post_refresh))
        .route("/discovery", get(handlers::discovery::get_discovery))
        .route("/audit", get(handlers::audit::get_audit))
        .route("/pipeline", get(handlers::pipeline::get_pipeline))
        .route("/reports", get(handlers::reports::get_reports))
        .route("/reports/export", get(handlers::reports::get_report_pdf))
        .route("/map", get(handlers::map::get_map))
        .route("/email/send", post(handlers::email::post_send))
        .route(
            "/leads",
            get(handlers::leads::get_leads).post(handlers::leads::post_lead),
        )
        .route("/jobs", post(handlers::jobs::post_job))
        .route("/queue/stats", get(handlers::jobs::get_queue_stats))
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

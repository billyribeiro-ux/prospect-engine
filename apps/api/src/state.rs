use sqlx::SqlitePool;

/// Shared application state for Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub jwt_secret: String,
}

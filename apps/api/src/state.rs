use std::sync::Arc;

use queue::MemoryQueue;
use sqlx::SqlitePool;

/// Shared application state for Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub jwt_secret: String,
    /// In-memory job queue (Phase 2 wiring; replace with durable queue in production).
    pub job_queue: Arc<MemoryQueue>,
}

impl AppState {
    /// Builds state for integration tests (in-memory DB wired separately).
    #[must_use]
    pub fn for_tests(pool: SqlitePool, jwt_secret: impl Into<String>) -> Self {
        Self {
            pool,
            jwt_secret: jwt_secret.into(),
            job_queue: Arc::new(MemoryQueue::new()),
        }
    }
}

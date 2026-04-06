use std::sync::Arc;

use queue::MemoryQueue;
use sqlx::AnyPool;

use crate::config::SmtpConfig;
use crate::services::rate_limit::MinuteWindowLimiter;

/// Shared application state for Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: AnyPool,
    pub jwt_secret: String,
    /// In-memory job queue (Phase 2 wiring; replace with durable queue in production).
    pub job_queue: Arc<MemoryQueue>,
    /// When set, email handler sends via SMTP; otherwise stub + persistence only.
    pub smtp: Option<SmtpConfig>,
    /// Rolling per-minute cap on `POST /email/send` (keyed by `X-Forwarded-For` or `direct`).
    pub email_rate: Arc<MinuteWindowLimiter>,
    /// Optional prefix for absolute tracking URLs in JSON.
    pub public_api_origin: Option<String>,
}

impl AppState {
    /// Builds state for integration tests (in-memory DB wired separately).
    #[must_use]
    pub fn for_tests(pool: AnyPool, jwt_secret: impl Into<String>) -> Self {
        Self {
            pool,
            jwt_secret: jwt_secret.into(),
            job_queue: Arc::new(MemoryQueue::new()),
            smtp: None,
            email_rate: Arc::new(MinuteWindowLimiter::new(0)),
            public_api_origin: None,
        }
    }
}

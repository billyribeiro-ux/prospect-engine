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
    /// Env-based SMTP (`PE_SMTP_*`). Saved Settings SMTP overrides when enabled (see `smtp_settings`).
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
        Self::for_tests_with_email_rate(pool, jwt_secret, 0)
    }

    /// Like [`for_tests`](Self::for_tests), with a non-zero per-minute cap on `POST /email/send`.
    #[must_use]
    pub fn for_tests_with_email_rate(
        pool: AnyPool,
        jwt_secret: impl Into<String>,
        email_rate_per_min: u32,
    ) -> Self {
        Self {
            pool,
            jwt_secret: jwt_secret.into(),
            job_queue: Arc::new(MemoryQueue::new()),
            smtp: None,
            email_rate: Arc::new(MinuteWindowLimiter::new(email_rate_per_min as usize)),
            public_api_origin: None,
        }
    }
}

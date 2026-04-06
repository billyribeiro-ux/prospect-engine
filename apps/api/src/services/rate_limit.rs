//! Per-key sliding window (≈1 minute) for email send abuse prevention.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

use crate::errors::ApiError;

pub struct MinuteWindowLimiter {
    inner: Mutex<HashMap<String, VecDeque<Instant>>>,
    max_per_minute: usize,
}

impl MinuteWindowLimiter {
    #[must_use]
    pub fn new(max_per_minute: usize) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            max_per_minute,
        }
    }

    /// `max_per_minute == 0` disables limiting (always succeeds).
    pub async fn check(&self, key: &str) -> Result<(), ApiError> {
        if self.max_per_minute == 0 {
            return Ok(());
        }
        let mut g = self.inner.lock().await;
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(60);
        let v = g.entry(key.to_string()).or_default();
        v.retain(|t| *t > cutoff);
        if v.len() >= self.max_per_minute {
            return Err(ApiError::RateLimited);
        }
        v.push_back(now);
        Ok(())
    }
}

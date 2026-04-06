#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::collections::VecDeque;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

#[async_trait]
pub trait JobQueue: Send + Sync {
    async fn enqueue(&self, job_id: &str) -> Result<(), std::io::Error>;
}

/// In-memory FIFO queue for tests and local development.
pub struct MemoryQueue {
    inner: Arc<Mutex<VecDeque<String>>>,
}

impl MemoryQueue {
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn drain(&self) -> Vec<String> {
        let mut g = self.inner.lock().await;
        g.drain(..).collect()
    }

    /// Current number of jobs waiting (FIFO depth).
    pub async fn depth(&self) -> usize {
        let g = self.inner.lock().await;
        g.len()
    }
}

impl Default for MemoryQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl JobQueue for MemoryQueue {
    async fn enqueue(&self, job_id: &str) -> Result<(), std::io::Error> {
        let mut g = self.inner.lock().await;
        g.push_back(job_id.to_string());
        Ok(())
    }
}

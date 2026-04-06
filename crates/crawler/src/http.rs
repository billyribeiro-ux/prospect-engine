//! HTTP fetch for discovery (bounded request).

use std::time::Duration;

use reqwest::Client;

use crate::CrawlError;

const FETCH_TIMEOUT: Duration = Duration::from_secs(30);

/// Fetches a URL and returns the response body as UTF-8 text (best effort).
pub async fn fetch_url(url: &str) -> Result<String, CrawlError> {
    let client = Client::builder()
        .timeout(FETCH_TIMEOUT)
        .user_agent("ProspectEngine-crawler/0.1")
        .build()
        .map_err(|e| CrawlError::Fetch(e.to_string()))?;
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| CrawlError::Fetch(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(CrawlError::Fetch(format!("HTTP {}", resp.status())));
    }
    resp.text()
        .await
        .map_err(|e| CrawlError::Fetch(e.to_string()))
}

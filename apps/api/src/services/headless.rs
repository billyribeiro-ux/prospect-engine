//! Optional headless Chromium DOM dump (`PE_CHROME_BIN`, default `chromium`).

use tokio::task::spawn_blocking;
use tokio::time::{timeout, Duration};

/// Returns HTML after JS execution via Chrome `--dump-dom`, or an error string.
pub async fn dump_dom(page_url: &str) -> Result<String, String> {
    let page_url = page_url.to_string();
    let chrome_bin =
        std::env::var("PE_CHROME_BIN").unwrap_or_else(|_| "chromium".to_string());
    let handle = spawn_blocking(move || {
        let out = std::process::Command::new(&chrome_bin)
            .args([
                "--headless=new",
                "--no-sandbox",
                "--disable-gpu",
                "--disable-dev-shm-usage",
                "--dump-dom",
                &page_url,
            ])
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(format!(
                "chrome exited {}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            ));
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    });
    match timeout(Duration::from_secs(60), handle).await {
        Err(_) => Err("headless chrome timed out".to_string()),
        Ok(Ok(Ok(html))) => Ok(html),
        Ok(Ok(Err(e))) => Err(e),
        Ok(Err(e)) => Err(e.to_string()),
    }
}

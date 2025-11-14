use anyhow::{Context, Result};
use reqwest::blocking::{Client, Response};

/// Simple api wrapper around reqwest.
pub(crate) fn fetch(client: &Client, url: &str) -> Result<Response> {
    let response = client
        .get(url)
        .send()
        .context(format!("Failed to send request to {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Request failed: {} (Is your session cookie valid?)",
            response.status()
        );
    }

    Ok(response)
}

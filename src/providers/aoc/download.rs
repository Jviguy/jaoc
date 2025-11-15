use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::utils::net;

/// Creates a client. Can fail if the session string is invalid.
pub(crate) fn make_client(session: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();
    let cookie_value = HeaderValue::from_str(&format!("session={}", session))
        .context("Failed to create HeaderValue from session (invalid characters?)")?;

    headers.insert("Cookie", cookie_value);

    let client = Client::builder()
        .user_agent("github.com/Jviguy/jaoc, jaoc scaffolding tool.")
        .default_headers(headers)
        .build()
        .context("Failed to build reqwest client")?;

    Ok(client)
}

/// Fetches the input for a given day using a client reference.
pub(crate) fn get_input(client: &Client, year: &str, day: u8) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    net::fetch(client, &url)?
        .text()
        .context("Failed to extract string, did the API change?")
}

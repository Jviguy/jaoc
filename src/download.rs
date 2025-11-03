use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::fs;
use std::path::Path;

/// Creates a client. Can fail if the session string is invalid.
pub fn make_client(session: &str) -> Result<Client> {
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
pub fn fetch_input(client: &Client, year: &str, day: u8) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let response = client.get(&url).send()
        .context(format!("Failed to send request to {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!("Request failed: {} (Is your session cookie valid?)", response.status());
    }

    let text = response.text()
        .context("Failed to extract text from response")?;

    Ok(text)
}

/// The main public function your `main.rs` will call.
pub fn download(year: String, day: u8) -> Result<()> {
    dotenv::dotenv().ok();
    // we care if we cant actually find the var. the Ok is fine
    // as the user might not have set the var in a .env.
    let session = std::env::var("AOC_SESSION")
        .context("AOC_SESSION not found in environment or .env file")?;

    let client = make_client(&session)
        .context("Failed to create HTTP client")?;

    println!("Downloading input for Year {}, Day {}...", year, day);
    let input = fetch_input(&client, &year, day)?;

    let day_name = format!("day{:02}", day);
    let path = Path::new("./data/inputs").join(format!("{}.txt", day_name));

    fs::write(&path, input)
        .context(format!("Failed to write input to file: {:?}", path))?;

    println!("✅ Successfully saved input to {:?}", path);
    Ok(())
}
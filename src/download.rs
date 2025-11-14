use aes::Aes256;
use aes::cipher::block_padding::{Pkcs7, UnpadError};
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use anyhow::{Context, Result};
use cbc::Decryptor;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Simple api wrapper around reqwest.
fn fetch(client: &Client, url: &str) -> Result<Response> {
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

/// Creates a client. Can fail if the session string is invalid.
fn make_aoc_client(session: &str) -> Result<Client> {
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
fn fetch_aoc_input(client: &Client, year: &str, day: u8) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    fetch(client, &url)?
        .text()
        .context("Failed to extract string, did the API change?")
}

pub fn aoc_download(year: &str, day: u8) -> Result<()> {
    dotenv::dotenv().ok();
    // we care if we cant actually find the var. the Ok is fine
    // as the user might not have set the var in a .env.
    let session = std::env::var("AOC_SESSION")
        .context("AOC_SESSION not found in environment or .env file")?;

    let client = make_aoc_client(&session).context("Failed to create HTTP client")?;

    println!("🚀 Downloading input for Year {}, Day {}...", year, day);
    let input = fetch_aoc_input(&client, &year, day)?;

    let day_name = format!("day{:02}", day);
    let path = Path::new("./data/inputs").join(format!("{}.txt", day_name));

    fs::write(&path, input).context(format!("Failed to write input to file: {:?}", path))?;

    println!("✅ Successfully saved input to {:?}", path);
    Ok(())
}

/// Creates a client. Can fail if the session string is invalid.
fn make_ebc_client(session: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();
    let cookie_value = HeaderValue::from_str(&format!("everybody-codes={}", session))
        .context("Failed to create HeaderValue from session (invalid characters?)")?;

    headers.insert("Cookie", cookie_value);

    let client = Client::builder()
        .user_agent("github.com/Jviguy/jaoc, jaoc scaffolding tool.")
        .default_headers(headers)
        .build()
        .context("Failed to build reqwest client")?;

    Ok(client)
}

fn ebc_get_key(client: &Client, year: &str, day: u8, part: u8) -> Result<Option<String>> {
    let mut keys: HashMap<String, String> = fetch(
        client,
        &format!("https://everybody.codes/api/event/{}/quest/{}", year, day),
    )?
    .json()
    .context("Failed to parse into proper representation, did the API change?")?;
    Ok(keys.remove(&format!("key{}", part.to_string())))
}

fn ebc_get_encrypted_inp(
    client: &Client,
    year: &str,
    day: u8,
    part: u8,
    seed: &str,
) -> Result<Option<String>> {
    let mut inputs: HashMap<String, String> = fetch(
        client,
        &format!(
            "https://everybody.codes/api/event/{}/quest/{}/{}.json",
            year, day, seed
        ),
    )?
    .json()
    .context("Failed to parse into proper representation, did the API change?")?;
    Ok(inputs.remove(part.to_string().as_str()))
}

type Aes256CbcDec = Decryptor<Aes256>;

fn ebc_decrypt_stuff(key: &str, input_hex: &str) -> Result<String> {
    let mut data_buffer = hex::decode(input_hex).context("Failed to decode hex input data")?;

    let key_bytes = key.as_bytes();

    // The key must be 32 bytes for AES-256
    if key_bytes.len() != 32 {
        return Err(anyhow::anyhow!(
            "Key must be 32 bytes (UTF-8 chars), but was {}",
            key_bytes.len()
        ));
    }

    let iv_bytes = &key_bytes[0..16];

    let decryptor = Aes256CbcDec::new(key_bytes.into(), iv_bytes.into());

    let plain_bytes = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut data_buffer)
        .map_err(|e: UnpadError| anyhow::anyhow!("Decryption/padding failed: {}", e))?;
    let s = str::from_utf8(plain_bytes).context("Invalid characters decrypted from input.")?;
    Ok(s.to_string())
}

fn fetch_ebc_input(client: &Client, year: &str, day: u8, part: u8, seed: &str) -> Result<String> {
    let key = ebc_get_key(client, year, day, part).context("Failed to get key: ")?;
    let Some(key) = key else {
        anyhow::bail!("Invalid part, or you haven't unlocked the part yet, key not available.");
    };
    let encrypted_input = ebc_get_encrypted_inp(client, year, day, part, seed)
        .context("Failed to fetch encrypted part.")?;
    let Some(encrypted_input) = encrypted_input else {
        anyhow::bail!("Invalid part, or you haven't unlocked the part yet, key not available.");
    };
    ebc_decrypt_stuff(&key, &encrypted_input)
}

pub fn ebc_download(year: &str, day: u8, part: u8) -> Result<()> {
    dotenv::dotenv().ok();
    // we care if we cant actually find the var. the Ok is fine
    // as the user might not have set the var in a .env.
    let session =
        std::env::var("EBC_ID").context("EBC_ID not found in environment or .env file")?;

    let seed =
        std::env::var("EBC_SEED").context("EBC_SEED not found in environment or .env file")?;

    let client = make_ebc_client(&session).context("Failed to create HTTP client")?;

    println!(
        "🚀 Downloading input for Year {}, Day {} Part {}...",
        year, day, part
    );
    let input = fetch_ebc_input(&client, &year, day, part, &seed)?;

    let path_name = format!("./data/inputs/day{:02}/{}.txt", day, part);
    let path = Path::new(path_name.as_str());

    fs::write(&path, input).context(format!("Failed to write input to file: {:?}", path))?;

    println!("✅ Successfully saved input to {:?}", path);
    Ok(())
}

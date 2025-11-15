use std::collections::HashMap;

use crate::utils::net;
use aes::Aes256;
use aes::cipher::block_padding::{Pkcs7, UnpadError};
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use anyhow::{Context, Result};
use cbc::Decryptor;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

pub fn make_client(session: &str) -> Result<Client> {
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

fn get_aes_key(client: &Client, year: &str, day: u8, part: u8) -> Result<Option<String>> {
    fetch_json_map_value(
        client,
        &format!("https://everybody.codes/api/event/{}/quest/{}", year, day),
        format!("key{}", part).as_str(),
    )
}

fn get_encrypted_part(
    client: &Client,
    year: &str,
    day: u8,
    part: u8,
    seed: &str,
) -> Result<Option<String>> {
    fetch_json_map_value(
        client,
        &format!(
            "https://everybody-codes.b-cdn.net/assets/{}/{}/input/{}.json",
            year, day, seed
        ),
        part.to_string().as_str(),
    )
}

fn fetch_json_map_value(client: &Client, url: &str, value: &str) -> Result<Option<String>> {
    let mut inputs: HashMap<String, String> = net::fetch(client, url)?
        .json()
        .context("Failed to parse into proper representation, did the API change?")?;
    Ok(inputs.remove(value))
}

type Aes256CbcDec = Decryptor<Aes256>;

fn decrypt(key: &str, input_hex: &str) -> Result<String> {
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

pub(crate) fn get_input(
    client: &Client,
    year: &str,
    day: u8,
    part: u8,
    seed: &str,
) -> Result<String> {
    let key = get_aes_key(client, year, day, part).context("Failed to get key: ")?;
    let Some(key) = key else {
        anyhow::bail!("Invalid part, or you haven't unlocked the part yet, key not available.");
    };
    let encrypted_input = get_encrypted_part(client, year, day, part, seed)
        .context("Failed to fetch encrypted part.")?;
    let Some(encrypted_input) = encrypted_input else {
        anyhow::bail!("Invalid part, or you haven't unlocked the part yet, key not available.");
    };
    decrypt(&key, &encrypted_input)
}

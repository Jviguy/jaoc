use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Jaoc {
    pub year: String,
    pub last_day: u8,
    pub auto_downloads: bool,
}

const PATH: &str = ".jaoc.toml";

/// Reads the .jaoc.toml config from the current directory.
/// Returns an error if the file is missing or corrupted.
pub fn read_config() -> Result<Jaoc> {
    println!("Reading config...");
    let bytes = fs::read(PATH)
        .context(format!("Failed to read config file at {}", PATH))?;

    let config = toml::from_slice(&bytes)
        .context(".jaoc.toml file is corrupted or malformed")?;

    Ok(config)
}

/// Serializes the config and writes it to a file in the specified directory.
/// `dir_path` is the *directory* where the .jaoc.toml file will be created.
pub fn write_config(config: &Jaoc, dir_path: &str) -> Result<()> {
    let toml_string = toml::to_string(config)
        .context("Failed to serialize config to TOML string")?;

    let full_path = Path::new(dir_path).join(PATH);

    fs::write(&full_path, toml_string)
        .context(format!("Failed to write config file to {:?}", full_path))?;

    Ok(())
}
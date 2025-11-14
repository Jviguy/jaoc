use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ProjectState {
    // Leave this here for now. Might need to use this with the new changes of 2 puzzles each day.
    AoC {},
    EBC { last_part: u8 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub year: String,
    pub auto_downloads: bool,

    pub last_day: u8,
    #[serde(flatten)]
    pub state: ProjectState,
}

const PATH: &str = ".jaoc.toml";

/// Reads the .jaoc.toml config from the current directory.
/// Returns an error if the file is missing or corrupted.
pub fn read() -> Result<Config> {
    println!("Reading config...");
    let bytes = fs::read(PATH).context(format!(
        "Failed to read config file, ./{}\n. \
        Run this command in your project root or if you have lost the file, \
        use jaoc regen.",
        PATH
    ))?;

    let config = toml::from_slice(&bytes).context(".jaoc.toml file is corrupted or malformed")?;

    Ok(config)
}
impl Config {
    /// Serializes the config and writes it to a file in the specified directory.
    /// `dir_path` is the *directory* where the .jaoc.toml file will be created.
    pub fn write(&self, dir_path: &str) -> Result<()> {
        let toml_string =
            toml::to_string(self).context("Failed to serialize config to TOML string")?;

        let full_path = Path::new(dir_path).join(PATH);

        fs::write(&full_path, toml_string)
            .context(format!("Failed to write config file to {:?}", full_path))?;

        Ok(())
    }
}

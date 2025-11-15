mod download;

use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::utils::files;

pub const REPO_TEMPLATE: &str = "https://github.com/jviguy/ebc-template.git";

const DAY_TEMPLATE: &str = include_str!("../../../templates/ebc/day.rs");

pub fn scaffold(day: u8) -> Result<()> {
    // Read for project-name from the cargo toml.
    let cargo_toml = std::fs::read_to_string("Cargo.toml")
        .context("Could not find Cargo.toml. Are you in the project root?")?;

    let crate_name = cargo_toml
        .lines()
        .find(|line| line.starts_with("name ="))
        .map(|line| line.split('=').nth(1).unwrap_or("").trim().replace('"', ""))
        .ok_or_else(|| anyhow::anyhow!("Failed to parse crate name from Cargo.toml"))?;

    let formatted_content = DAY_TEMPLATE
        .replace("{{project-name}}", &crate_name)
        .replace("{{day}}", day.to_string().as_str());

    let day_name = format!("day{:02}", day);
    let bin_path = std::path::PathBuf::from("./src/bin").join(format!("{}.rs", day_name));

    files::write_to_bin_file(&bin_path, &formatted_content)?;

    // Set up the empty input/day01 folder and etc this is different from AoC.
    let input_path = std::path::PathBuf::from("./data/inputs").join(day_name.as_str());
    let example_path = std::path::PathBuf::from("./data/examples").join(day_name.as_str());

    std::fs::create_dir_all(input_path).context("Failed to create inputs dir")?;
    std::fs::create_dir_all(example_path).context("Failed to create examples dir")?;

    Ok(())
}

pub fn download(year: &str, day: u8, part: u8) -> Result<()> {
    dotenv::dotenv().ok();
    // we care if we cant actually find the var. the Ok is fine
    // as the user might not have set the var in a .env.
    let session =
        std::env::var("EBC_ID").context("EBC_ID not found in environment or .env file")?;

    let seed =
        std::env::var("EBC_SEED").context("EBC_SEED not found in environment or .env file")?;

    let client = download::make_client(&session).context("Failed to create HTTP client")?;

    println!(
        "🚀 Downloading input for Year {}, Day {} Part {}...",
        year, day, part
    );
    let input = download::get_input(&client, year, day, part, &seed)?;

    let path_name = format!("./data/inputs/day{:02}/{}.txt", day, part);
    let path = Path::new(path_name.as_str());

    fs::write(path, input).context(format!("Failed to write input to file: {:?}", path))?;

    println!("✅ Successfully saved input to {:?}", path);
    Ok(())
}

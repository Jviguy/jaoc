mod download;

use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::utils::{config::read_crate_name, files};

const DAY_TEMPLATE: &str = include_str!("../../../templates/aoc/day.rs");

pub fn scaffold(day: u8) -> Result<()> {
    let crate_name = read_crate_name()?;

    let formatted_content = DAY_TEMPLATE
        .replace("{{project-name}}", &crate_name)
        .replace(
            "aoc_main!(1, part1, part2)",
            format!("aoc_main!({}, part1, part2)", &day.to_string()).as_str(),
        );
    let day_name = format!("day{:02}", day);
    let bin_path = std::path::PathBuf::from("./src/bin").join(format!("{}.rs", day_name));

    files::write_to_bin_file(&bin_path, &formatted_content)?;

    // Create the empty input and example files so that we can later download into them
    // or have the user paste into them.
    let input_path = std::path::PathBuf::from("./data/inputs").join(format!("{}.txt", day_name));
    let example_path =
        std::path::PathBuf::from("./data/examples").join(format!("{}.txt", day_name));

    std::fs::create_dir_all("./data/inputs").context("Failed to create inputs dir")?;
    std::fs::create_dir_all("./data/examples").context("Failed to create examples dir")?;

    files::create_empty_file(&input_path);
    files::create_empty_file(&example_path);
    Ok(())
}

pub fn download(year: &str, day: u8) -> Result<()> {
    dotenv::dotenv().ok();
    // we care if dotenv fails var. the Ok is fine
    // as the user might not have set the var in a .env.
    let session = std::env::var("AOC_SESSION")
        .context("AOC_SESSION not found in environment or .env file")?;

    let client = download::make_client(&session).context("Failed to create HTTP client")?;

    println!("🚀 Downloading input for Year {}, Day {}...", year, day);
    let input = download::get_input(&client, year, day)?;

    let day_name = format!("day{:02}", day);
    let path = Path::new("./data/inputs").join(format!("{}.txt", day_name));

    fs::write(&path, input).context(format!("Failed to write input to file: {:?}", path))?;

    println!("✅ Successfully saved input to {:?}", path);
    Ok(())
}

mod config;
mod download;

use crate::config::{read_config, write_config, Jaoc};
use anyhow::Context;
use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "jaoc")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        name: String,
        year: String,
    },
    Start {
        day: u8,
    },
    Download {
        day: u8,
    },

}

// include the template file for days at comp time.
const DAY_TEMPLATE: &'static str = include_str!("../aoc-template/src/bin/day01.rs");

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, year } => {
            let template_repo = "https://github.com/jviguy/aoc-template.git";

            let status = Command::new("cargo")
                .args([
                    "generate",
                    "--git", template_repo,
                    "--name", &name,
                ])
                .status()
                .context("Failed to execute cargo-generate. Is it installed?")?;

            // Use anyhow::bail! to create a new error if the command failed
            if !status.success() {
                anyhow::bail!("cargo-generate failed to create the project.");
            }

            println!("Successfully created!");
            let config = Jaoc {
                year
            };

            write_config(&config, &format!("./{}/", name))
                .context("Failed to write .jaoc.toml config file")?;
        }
        Commands::Start { day } => {
            let cargo_toml = fs::read_to_string("Cargo.toml")
                .context("Could not find Cargo.toml. Are you in the project root?")?;

            let crate_name = cargo_toml.lines()
                .find(|line| line.starts_with("name ="))
                .map(|line| line.split('=').nth(1).unwrap_or("").trim().replace('"', ""))
                .ok_or_else(|| anyhow::anyhow!("Failed to parse crate name from Cargo.toml"))?;

            let formatted_content = DAY_TEMPLATE
                .replace("{{project-name}}", &crate_name)
                .replace("aoc_main!(1, part1, part2)", format!("aoc_main!({}, part1, part2)", &day.to_string()).as_str());

            let day_name = format!("day{:02}", day);
            let bin_path = PathBuf::from("./src/bin").join(format!("{}.rs", day_name));

            match File::create_new(&bin_path) {
                Ok(mut file) => {
                    file.write_all(formatted_content.as_bytes())
                        .context(format!("Failed to write to bin file: {:?}", bin_path))?;
                    println!("✅ Created binary: {:?}", bin_path);
                }
                Err(e) if e.kind() == AlreadyExists => {
                    eprintln!("⚠️ Binary file already exists: {:?}", bin_path);
                }
                Err(e) => {
                    return Err(e).context(format!("Failed to create bin file: {:?}", bin_path));
                }
            }

            let input_path = PathBuf::from("./data/inputs").join(format!("{}.txt", day_name));
            let example_path = PathBuf::from("./data/examples").join(format!("{}.txt", day_name));

            fs::create_dir_all("./data/inputs")
                .context("Failed to create inputs dir")?;
            fs::create_dir_all("./data/examples")
                .context("Failed to create examples dir")?;

            create_empty_file(&input_path);
            create_empty_file(&example_path);
        }
        Commands::Download { day } => {
            println!("Reading config...");
            let config = read_config()?;

            download::download(config.year, day)?;
        }
    }
    Ok(())
}

fn create_empty_file(path: &PathBuf) {
    match File::create_new(path) {
        Ok(_) => println!("✅ Created data file: {:?}", path),
        Err(e) if e.kind() == AlreadyExists => {
            eprintln!("⚠️ Data file already exists: {:?}", path);
        }
        Err(e) => eprintln!("🔥 Failed to create data file: {}", e),
    }
}
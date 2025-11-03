use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::{Write};
use std::path::PathBuf;
use std::process::Command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jaoc")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        year: String,
    },
    Start {
        day: u8,
    }
}

// include the template file for days at comp time.
const DAY_TEMPLATE: &'static str = include_str!("../aoc-template/src/bin/day01.rs");

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { year } => {
            let project_name = format!("aoc_{}", year);
            let template_repo = "https://github.com/jviguy/aoc-template.git"; // <-- YOUR REPO

            println!("Setting up new AoC project: {}...", project_name);

            let status = Command::new("cargo")
                .args([
                    "generate",
                    "--git", template_repo,
                    "--name", &project_name, // This passes the {{project_name}} variable
                ])
                .status()
                .expect("Failed to execute cargo-generate");

            if status.success() {
                println!("Successfully created {}!", project_name);
                println!("Run `cd {}` to get started.", project_name);
            } else {
                eprintln!("Failed to create project. Is cargo-generate installed?");
            }
        },
        Commands::Start { day } => {
            let cargo_toml = fs::read_to_string("Cargo.toml")
                .expect("Could not find Cargo.toml. Are you in the project root?");
            // maybe later actually parse this properly but meh.
            let crate_name = cargo_toml.lines()
                .find(|line| line.starts_with("name ="))
                .map(|line| line.split('=').nth(1).unwrap_or("").trim().replace('"', ""))
                .expect("Failed to parse crate name from Cargo.toml");

            let formatted_content = DAY_TEMPLATE
                .replace("{{project_name}}", &crate_name)
                .replace("{{DAY_NUM_PLACEHOLDER}}", day.to_string().as_str());

            let day_name = format!("day{:02}", day);
            let bin_path = PathBuf::from("./src/bin").join(format!("{}.rs", day_name));

            // the ai emojis are fire. best use of clankers is nice logging messages lmao.
            match File::create_new(&bin_path) {
                Ok(mut file) => {
                    file.write_all(formatted_content.as_bytes()).expect("Failed to write to bin file.");
                    println!("✅ Created binary: {:?}", bin_path);
                }
                Err(e) if e.kind() == AlreadyExists => {
                    eprintln!("⚠️ Binary file already exists: {:?}", bin_path);
                }
                Err(e) => {
                    eprintln!("🔥 Failed to create bin file: {}", e);
                }
            }

            let input_path = PathBuf::from("./data/inputs").join(format!("{}.txt", day_name));
            let example_path = PathBuf::from("./data/examples").join(format!("{}.txt", day_name));

            fs::create_dir_all("./data/inputs").expect("Failed to create inputs dir");
            fs::create_dir_all("./data/examples").expect("Failed to create examples dir");

            create_empty_file(&input_path);
            create_empty_file(&example_path);
        }
    }
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
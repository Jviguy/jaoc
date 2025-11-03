use std::fs;
use std::fs::{File};
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
        name: String,
        year: String,
    },
    Start {
        day: u8,
    },
}

// include the template file for days at comp time.
const DAY_TEMPLATE: &'static str = include_str!("../aoc-template/src/bin/day01.rs");

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, year: _year } => {
            let template_repo = "https://github.com/jviguy/aoc-template.git"; // <-- YOUR REPO

            let status = Command::new("cargo")
                .args([
                    "generate",
                    "--git", template_repo,
                    "--name", name,
                ])
                .status()
                .expect("Failed to execute cargo-generate");

            if status.success() {
                println!("Successfully created!");
            } else {
                eprintln!("Failed to create project. Is cargo-generate installed?");
            }
        },
        Commands::Start { day } => {
            let cargo_toml = fs::read_to_string("Cargo.toml")
                .expect("Could not find Cargo.toml. Are you in the project root?");

            let crate_name = cargo_toml.lines()
                .find(|line| line.starts_with("name ="))
                .map(|line| line.split('=').nth(1).unwrap_or("").trim().replace('"', ""))
                .expect("Failed to parse crate name from Cargo.toml");

            let formatted_content = DAY_TEMPLATE
                .replace("{{project-name}}", crate_name.as_str())
                .replace("aoc_main!(1, part1, part2)", format!("aoc_main!({}, part1, part2)", day.to_string()).as_str());


            let day_name = format!("day{:02}", day);
            let bin_path = PathBuf::from("./src/bin").join(format!("{}.rs", day_name));

            // the AI emojis are fire. best use of clankers is nice logging messages lmao.
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
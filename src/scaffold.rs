use crate::download;
use anyhow::Context;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;
use std::path::PathBuf;

// include the template file for days at comp time.
const DAY_TEMPLATE: &str = include_str!("../aoc-template/src/bin/day01.rs");

pub fn day_download(d: u8, year: &str) -> anyhow::Result<()> {
    day(d)?;
    download::download(year, d)?;
    Ok(())
}

pub fn day(day: u8) -> anyhow::Result<()> {
    // Read for project-name from the cargo toml.
    let cargo_toml = std::fs::read_to_string("Cargo.toml")
        .context("Could not find Cargo.toml. Are you in the project root?")?;

    let crate_name = cargo_toml.lines()
        .find(|line| line.starts_with("name ="))
        .map(|line| line.split('=').nth(1).unwrap_or("").trim().replace('"', ""))
        .ok_or_else(|| anyhow::anyhow!("Failed to parse crate name from Cargo.toml"))?;

    let formatted_content = DAY_TEMPLATE
        .replace("{{project-name}}", &crate_name)
        .replace("aoc_main!(1, part1, part2)", format!("aoc_main!({}, part1, part2)", &day
            .to_string()).as_str());

    let day_name = format!("day{:02}", day);
    let bin_path = std::path::PathBuf::from("./src/bin").join(format!("{}.rs", day_name));

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

    let input_path = std::path::PathBuf::from("./data/inputs").join(format!("{}.txt",
                                                                            day_name));
    let example_path = std::path::PathBuf::from("./data/examples").join(format!("{}.txt",
                                                                                day_name));

    std::fs::create_dir_all("./data/inputs")
        .context("Failed to create inputs dir")?;
    std::fs::create_dir_all("./data/examples")
        .context("Failed to create examples dir")?;

    create_empty_file(&input_path);
    create_empty_file(&example_path);
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

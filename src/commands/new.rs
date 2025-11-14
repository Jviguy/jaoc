use crate::{
    commands::JaocCommand,
    utils::config::{Config, ProjectState},
};
use anyhow::Context;
use clap::{Args, ValueEnum};
use std::process::Command;

const AOC_TEMPLATE_REPO: &str = "https://github.com/jviguy/aoc-template.git";
const EBC_TEMPLATE_REPO: &str = "https://github.com/jviguy/ebc-template.git";

#[derive(Debug, Clone, ValueEnum)]
pub enum ProjectType {
    AoC,
    EbC,
}

#[derive(Args)]
pub struct NewArgs {
    pub name: String,
    pub year: String,
    #[arg(long = "type", default_value = "aoc")]
    pub t: ProjectType,
}

impl JaocCommand for NewArgs {
    fn execute(self) -> anyhow::Result<()> {
        let template = match self.t {
            ProjectType::AoC => AOC_TEMPLATE_REPO,
            ProjectType::EbC => EBC_TEMPLATE_REPO,
        };
        let status = Command::new("cargo")
            .args(["generate", "--git", template, "--name", &self.name])
            .status()
            .context("Failed to execute cargo-generate. Is it installed?")?;

        // Use anyhow::bail! to create a new error if the command failed
        if !status.success() {
            anyhow::bail!("cargo-generate failed to create the project.");
        }

        let state = match self.t {
            ProjectType::AoC => ProjectState::AoC {},
            ProjectType::EbC => ProjectState::EbC { last_part: 0 },
        };

        println!("🚀 Successfully created!");
        let config = Config {
            year: self.year,
            last_day: 1,
            auto_downloads: true,
            state,
        };

        config
            .write(&format!("./{}/", self.name))
            .context("Failed to write .jaoc.toml config file")?;
        Ok(())
    }
}

use crate::commands::JaocCommand;
use crate::config::{write_config, Jaoc};
use anyhow::Context;
use clap::Args;
use std::process::Command;

const TEMPLATE_REPO: &str = "https://github.com/jviguy/aoc-template.git";

#[derive(Args)]
pub struct NewArgs {
    pub name: String,
    pub year: String,
}

impl JaocCommand for NewArgs {
    fn execute(self) -> anyhow::Result<()> {
        let status = Command::new("cargo")
            .args([
                "generate",
                "--git", TEMPLATE_REPO,
                "--name", &self.name,
            ])
            .status()
            .context("Failed to execute cargo-generate. Is it installed?")?;

        // Use anyhow::bail! to create a new error if the command failed
        if !status.success() {
            anyhow::bail!("cargo-generate failed to create the project.");
        }

        println!("🚀 Successfully created!");
        let config = Jaoc {
            year: self.year,
            last_day: 1,
            auto_downloads: true,
        };

        write_config(&config, &format!("./{}/", self.name))
            .context("Failed to write .jaoc.toml config file")?;
        Ok(())
    }
}
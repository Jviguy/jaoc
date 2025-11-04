use crate::commands::JaocCommand;
use anyhow::Context;
use clap::Args;

#[derive(Args)]
pub struct RunArgs {
    pub day: u8,
    #[arg(long, short, default_value_t = 1)]
    pub part: u8,
    #[arg(long, short)]
    pub example: bool,
}

impl JaocCommand for RunArgs {
    fn execute(self) -> anyhow::Result<()> {
        let day_name = format!("day{:02}", self.day);

        let mut args = vec!["run", "--bin", &day_name, "--"];
        args.push(if self.part == 1 { "1" } else { "2" });
        if self.example {
            args.push("--example");
        }

        println!("🚀 Running: cargo {}", args.join(" "));

        std::process::Command::new("cargo")
            .args(&args)
            .status() // wait for it to finish.
            .context("Failed to run cargo command")?;
        Ok(())
    }
}
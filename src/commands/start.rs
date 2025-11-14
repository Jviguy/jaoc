use crate::commands::JaocCommand;
use crate::providers::{aoc, ebc};
use crate::utils::config::{ProjectState, read};
use clap::Args;

#[derive(Args)]
pub struct StartArgs {
    pub day: u8,
    pub part: Option<u8>,
}

impl JaocCommand for StartArgs {
    fn execute(self) -> anyhow::Result<()> {
        let config = read()?;
        println!("🚀 Setting up Day {}...", self.day);
        match config.state {
            ProjectState::AoC {} => aoc::scaffold(self.day),
            ProjectState::EbC { last_part: _ } => ebc::scaffold(self.day),
        }?;
        if config.auto_downloads {
            match config.state {
                ProjectState::AoC {} => aoc::download(&config.year, self.day),
                ProjectState::EbC { last_part } => {
                    ebc::download(&config.year, self.day, self.part.unwrap_or(last_part + 1))
                }
            }?;
        }
        println!("✅ All set for Day {}! Good luck!", self.day);
        Ok(())
    }
}

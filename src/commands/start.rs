use crate::commands::JaocCommand;
use crate::config::{ProjectState, read};
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
        if config.auto_downloads {
            match config.state {
                ProjectState::AoC {} => crate::scaffold::aoc_day_download(self.day, &config.year),
                ProjectState::EBC { last_part } => crate::scaffold::ebc_day_download(
                    self.day,
                    &config.year,
                    self.part.unwrap_or(last_part + 1),
                ),
            }
        } else {
            match config.state {
                ProjectState::AoC {} => crate::scaffold::aoc_day(self.day),
                ProjectState::EBC { last_part: _ } => crate::scaffold::ebc_day(self.day),
            }
        }?;
        println!("✅ All set for Day {}! Good luck!", self.day);
        Ok(())
    }
}

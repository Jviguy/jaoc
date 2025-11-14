use crate::commands::JaocCommand;
use crate::config::{ProjectState, read};
use clap::Args;

#[derive(Args)]
pub struct NextArgs {}

impl JaocCommand for NextArgs {
    fn execute(self) -> anyhow::Result<()> {
        let mut config = read()?;
        match config.state {
            ProjectState::AoC {} => {
                let next_day = config.last_day + 1;

                if next_day > 25 {
                    anyhow::bail!("🎉 You've already finished Day 25!");
                }

                println!("🚀 Setting up Day {}...", next_day);

                if config.auto_downloads {
                    crate::scaffold::aoc_day_download(next_day, &config.year)
                } else {
                    crate::scaffold::aoc_day(next_day)
                }?;

                config.last_day = next_day;
                config.write("./")?;

                println!("✅ All set for Day {}! Good luck!", next_day);
            }
            ProjectState::EBC { last_part } => {
                let mut next_part = last_part + 1;
                let mut next_day = config.last_day;
                if next_part > 3 {
                    next_part = 1;
                    next_day += 1;
                }

                if next_day > 25 {
                    anyhow::bail!("🎉 You've already finished Day 25!");
                }

                println!("🚀 Setting up Day {}, Part {}...", next_day, next_part);

                if config.auto_downloads {
                    crate::scaffold::ebc_day_download(next_day, &config.year)
                } else {
                    crate::scaffold::ebc_day(next_day)
                }?;

                config.last_day = next_day;
                config.state = ProjectState::EBC {
                    last_part: next_part,
                };
                config.write("./")?;

                println!(
                    "✅ All set for Day {} Part {}! Good luck!",
                    next_day, next_part
                );
            }
        }
        Ok(())
    }
}

use crate::commands::JaocCommand;
use crate::config::{read_config, write_config};
use clap::Args;

#[derive(Args)]
pub struct NextArgs {}

impl JaocCommand for NextArgs {
    fn execute(self) -> anyhow::Result<()> {
        let mut config = read_config()?;
        let next_day = config.last_day + 1;

        if next_day > 25 {
            anyhow::bail!("🎉 You've already finished Day 25!");
        }

        println!("🚀 Setting up Day {}...", next_day);

        if config.auto_downloads {
            crate::scaffold::day_download(next_day, &config.year)
        } else {
            crate::scaffold::day(next_day)
        }?;

        config.last_day = next_day;
        write_config(&config, "./")?;

        println!("✅ All set for Day {}! Good luck!", next_day);
        Ok(())
    }
}
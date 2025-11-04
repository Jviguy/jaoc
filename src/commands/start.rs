use crate::commands::JaocCommand;
use crate::config::read_config;
use clap::Args;

#[derive(Args)]
pub struct StartArgs {
    pub day: u8,
}


impl JaocCommand for StartArgs {
    fn execute(self) -> anyhow::Result<()> {
        let config = read_config()?;
        println!("🚀 Setting up Day {}...", self.day);
        if config.auto_downloads {
            crate::scaffold::day_download(self.day, &config.year)
        } else {
            crate::scaffold::day(self.day)
        }?;
        println!("✅ All set for Day {}! Good luck!", self.day);
        Ok(())
    }
}


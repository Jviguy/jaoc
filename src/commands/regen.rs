use crate::commands::JaocCommand;
use crate::config::{read_config, write_config, Jaoc};
use clap::Args;

#[derive(Args)]
pub struct RegenArgs {
    pub year: String,
    pub day: u8,
}

impl JaocCommand for RegenArgs {
    fn execute(self) -> anyhow::Result<()> {
        if read_config().is_ok() {
            anyhow::bail!("⚠️ Config file already exists and isn't malformed.");
        };
        let config = Jaoc {
            year: self.year,
            last_day: self.day,
            auto_downloads: true,
        };
        write_config(&config, "./")?;
        println!("✅ Successfully regenerated config, ensure that .jaoc.toml is correct.");
        Ok(())
    }
}
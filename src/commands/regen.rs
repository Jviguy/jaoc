use crate::commands::JaocCommand;
use crate::commands::new::ProjectType;
use crate::utils::config::{Config, ProjectState, read};
use clap::Args;

#[derive(Args)]
pub struct RegenArgs {
    pub year: String,
    pub day: u8,
    #[arg(long = "type", default_value = "aoc")]
    pub t: ProjectType,
}

impl JaocCommand for RegenArgs {
    fn execute(self) -> anyhow::Result<()> {
        if read().is_ok() {
            anyhow::bail!("⚠️ Config file already exists and isn't malformed.");
        };
        let state = match self.t {
            ProjectType::AoC => ProjectState::AoC {},
            ProjectType::EbC => ProjectState::EbC { last_part: 0 },
        };

        let config = Config {
            year: self.year,
            last_day: 1,
            auto_downloads: true,
            state,
        };
        config.write("./")?;
        println!("✅ Successfully regenerated config, ensure that .jaoc.toml is correct.");
        Ok(())
    }
}

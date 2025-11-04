use crate::commands::JaocCommand;
use crate::config::read_config;
use crate::download;
use clap::Args;

#[derive(Args)]
pub struct DownloadArgs {
    pub day: u8,
}

impl JaocCommand for DownloadArgs {
    fn execute(self) -> anyhow::Result<()> {
        let config = read_config()?;

        download::download(&config.year, self.day)?;
        Ok(())
    }
}
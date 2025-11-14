use crate::commands::JaocCommand;
use crate::config::{ProjectState, read};
use crate::download;
use clap::Args;

#[derive(Args)]
pub struct DownloadArgs {
    pub day: u8,
    // TODO: This kinda sucks, optional as could be EBC, but also can be seen as the same use case next. idk sucks.
    pub part: Option<u8>,
}

impl JaocCommand for DownloadArgs {
    fn execute(self) -> anyhow::Result<()> {
        let config = read()?;
        match config.state {
            ProjectState::AoC {} => download::aoc_download(&config.year, self.day),
            ProjectState::EBC { last_part } => {
                download::ebc_download(&config.year, self.day, self.part.unwrap_or(last_part + 1))
            }
        }?;
        Ok(())
    }
}

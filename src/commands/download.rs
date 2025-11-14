use crate::commands::JaocCommand;
use crate::providers::{aoc, ebc};
use crate::utils::config::{ProjectState, read};
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
            ProjectState::AoC {} => aoc::download(&config.year, self.day),
            ProjectState::EbC { last_part } => {
                ebc::download(&config.year, self.day, self.part.unwrap_or(last_part + 1))
            }
        }?;
        Ok(())
    }
}

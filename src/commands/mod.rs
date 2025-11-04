pub mod start;
pub mod new;
pub mod run;
pub mod next;
pub mod watch;
mod download;
mod regen;

use anyhow::Result;
use clap::Subcommand;

/// A trait for any executable jaoc subcommand
pub trait JaocCommand {
    fn execute(self) -> Result<()>;
}

#[derive(Subcommand)]
pub enum Commands {
    New(new::NewArgs),
    Start(start::StartArgs),
    Download(download::DownloadArgs),
    Run(run::RunArgs),
    Watch(watch::WatchArgs),
    Next(next::NextArgs),
    Regen(regen::RegenArgs),
}

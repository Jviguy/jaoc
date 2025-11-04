mod config;
mod download;
mod commands;
mod scaffold;

use crate::commands::{Commands, JaocCommand};
use clap::Parser;

#[derive(Parser)]
#[command(name = "jaoc")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New(args) => args.execute(),
        Commands::Start(args) => args.execute(),
        Commands::Download(args) => args.execute(),
        Commands::Run(args) => args.execute(),
        Commands::Watch(args) => args.execute(),
        Commands::Next(args) => args.execute()
    }?;
    Ok(())
}

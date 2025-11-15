pub(crate) mod commands;
pub(crate) mod providers;
pub(crate) mod utils;

use crate::commands::{Commands, JaocCommand};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "jaoc",
    about = "A scaffolding tool for rust projects solving Advent of Code.",
    version = env!("CARGO_PKG_VERSION"),
    author = "Jeremy, https://github.com/Jviguy/"
)]
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
        Commands::Next(args) => args.execute(),
        Commands::Regen(args) => args.execute(),
    }?;
    Ok(())
}

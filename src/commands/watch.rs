use crate::commands::JaocCommand;
use clap::Args;
use notify::Watcher;

#[derive(Args)]
pub struct WatchArgs {
    pub day: u8,
    #[arg(long, short, default_value_t = 1)]
    pub part: u8,
    #[arg(long, short)]
    pub example: bool,
}

impl JaocCommand for WatchArgs {
    fn execute(self) -> anyhow::Result<()> {
        let day_name = format!("day{:02}", self.day);

        let mut args = vec!["run", "--bin", &day_name, "--"];
        args.push(if self.part == 1 { "1" } else { "2" });
        if self.example {
            args.push("--example");
        }
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;

        // watch the source file. Not the best way as what if they change
        // some code in a different file but meh this is a basic assumption.
        // TODO: later make this more sophisticated.
        let path_to_watch = format!("./src/bin/day{:02}.rs", self.day);
        watcher.watch(std::path::Path::new(&path_to_watch),
                      notify::RecursiveMode::NonRecursive)?;

        println!("Watching {:?}... Press Ctrl+C to exit.", path_to_watch);

        // Run it once immediately
        std::process::Command::new("cargo").args(&args).status()?;
        // TODO: maybe implement actually signal listening here.
        for res in rx {
            if let Ok(notify::Event { kind: notify::EventKind::Modify(_), .. }) = res {
                println!("File changed, re-running...");
                std::process::Command::new("cargo").args(&args).status()?;
            }
        }
        Ok(())
    }
}
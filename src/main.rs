use std::path::PathBuf;

use clap::{Parser, Subcommand};
use native_db::Builder;
use wq::{
    MODELS,
    event::{Event, EventKind},
};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long, env = "WG_DATABASE", default_value = "~/.wg")]
    data_dir: PathBuf,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Command {
    /// Add new task to the work queue
    Add { key: String },
    /// Start working on task
    Start { key: String },
    /// Stop working on task
    Stop { key: String },
    /// Finnish task and remove it from queue
    Finnish { key: String },

    /// Emit all events as JSON
    Dump,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let db = Builder::new().create(&*MODELS, cli.data_dir.as_path())?;

    let r = db.r_transaction()?;
    let events = r
        .scan()
        .primary()?
        .all()?
        .collect::<Result<Vec<Event>, _>>()?;

    if let Some(new_event) = match cli.command {
        Command::Add { ref key } => Some(Event::new(key.clone(), EventKind::Enqueue)),
        Command::Start { ref key } => Some(Event::new(key.clone(), EventKind::Start)),
        Command::Stop { ref key } => Some(Event::new(key.clone(), EventKind::Stop)),
        Command::Finnish { ref key } => Some(Event::new(key.clone(), EventKind::Dequeue)),
        Command::Dump => None,
    } {
        let rw = db.rw_transaction()?;
        rw.insert(new_event)?;
        rw.commit()?;
    };

    if cli.command == Command::Dump {
        print!("{events:?}")
    };

    Ok(())
}

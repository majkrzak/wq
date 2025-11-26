use std::path::PathBuf;

use clap::{Parser, Subcommand};
use native_db::Builder;
use wq::{
    MODELS,
    event::{Event, EventKind},
    state::{State, TransitionError},
};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
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
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let db = Builder::new().create(&*MODELS, cli.data_dir.as_path())?;

    let rw = db.rw_transaction()?;
    let events = rw
        .scan()
        .primary()?
        .all()?
        .collect::<Result<Vec<Event>, _>>()?;

    let state = events
        .iter()
        .fold(Ok::<State, TransitionError>(State::new()), |s, e| {
            s?.apply(e)
        })
        .expect("State incorrect");

    println!("{state:?}");

    if let Some(new_event) = match cli.command {
        Some(Command::Add { ref key }) => Some(Event::new(key.clone(), EventKind::Enqueue)),
        Some(Command::Start { ref key }) => Some(Event::new(key.clone(), EventKind::Start)),
        Some(Command::Stop { ref key }) => Some(Event::new(key.clone(), EventKind::Stop)),
        Some(Command::Finnish { ref key }) => Some(Event::new(key.clone(), EventKind::Dequeue)),
        _ => None,
    } {
        let new_state = state.apply(&new_event).expect("Invalid new state");
        println!("{new_state:?}");
        rw.insert(new_event)?;
    };

    rw.commit()?;

    Ok(())
}

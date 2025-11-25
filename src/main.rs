use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
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

fn main() {
    let cli = Cli::parse();
    print!("{cli:?}")
}

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, PartialEq)]
#[command()]
enum Command {
    Add,
    Start,
    Pause,
    Finish,
    Comment,
    Link,
}

#[derive(Subcommand, Debug, PartialEq)]
enum OtherCommand {
    Test,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    println!("{cli:?}");
    Ok(())
}

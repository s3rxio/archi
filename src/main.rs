mod command;

use clap::Parser;
use command::{Command, Commands};

#[derive(Parser, Debug)]
#[command(
    author = "s3rxio",
    version = "0.0.0",
    about = "Archlinux installer tool",
    long_about = "Archlinux installer tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> anyhow::Result<(), anyhow::Error> {
    let args = Cli::parse();

    // FIXME: How to do that without match or without writing each command
    match args.command {
        Commands::Partition(command) => command.execute(),
    }
}

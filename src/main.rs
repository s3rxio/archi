mod partition;

use clap::{Parser, Subcommand};
use partition::PartitionCommand;

#[derive(Parser, Debug)]
#[command(
    author = "s3rxio",
    version = "0.0.0",
    about = "Archlinux installer tool",
    long_about = "Archlinux installer tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(subcommand)]
    Partition(PartitionCommand),
}

fn main() {
    let args = Cli::parse();

    dbg!(&args);

    // FIXME: How to do that without match or without writing each command
    match args.command {
        Command::Partition(command) => command.execute().unwrap(),
    }
}

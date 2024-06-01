use anyhow::Result;
use clap::Subcommand;

mod partition;
use partition::PartitionCommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Partition(PartitionCommand),
}

pub trait Command {
    fn execute(&self) -> Result<()>;
}

use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
#[command(about = "Manage partitions")]
pub enum PartitionCommand {
    #[command(about = "Creating a new partition")]
    New {
        /// Partition filesystem
        filesystem: String,

        /// Path to disk drive
        path: Option<PathBuf>,

        /// Partition size
        #[arg(short = 's', long = "size")]
        size: Option<String>,

        /// Partition mount point
        #[arg(short = 'm', long = "mount")]
        mount: Option<PathBuf>,
    },

    #[command(about = "Listing all partitions")]
    List { path: Option<PathBuf> },

    #[command(about = "Deleting a partition")]
    Delete { path: PathBuf },
}

// TODO: Implement partition commands
#[allow(unreachable_code, unused_variables)]
impl PartitionCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            PartitionCommand::New {
                filesystem,
                path,
                size,
                mount,
            } => todo!(),
            PartitionCommand::List { path } => todo!(),
            PartitionCommand::Delete { path } => todo!(),
        }

        Ok(())
    }
}

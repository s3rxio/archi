mod utils;

use super::Command;
use anyhow::Ok;
use clap::Subcommand;
use libparted::Device;
use std::path::PathBuf;
use utils::print_partitions;

#[derive(Subcommand, Debug, Clone)]
#[command(alias = "part", about = "Manage partitions")]
pub enum PartitionCommand {
    #[command(about = "Creating a new partition")]
    New {
        /// Partition path
        path: Option<PathBuf>,

        /// Partition filesystem
        #[arg(long = "fs", alias = "filesystem")]
        filesystem: String,

        /// FAT size
        #[arg(long = "fsize", alias = "fat-size")]
        fat_size: Option<u8>,

        /// Partition size
        #[arg(short = 's', long = "size")]
        size: Option<String>,

        /// Partition mount point
        #[arg(short = 'm', long = "mount")]
        mount: Option<PathBuf>,
    },

    #[command(alias = "ls", about = "Listing all partitions")]
    List {
        /// Partition path
        path: Option<PathBuf>,
    },

    #[command(about = "Deleting a partition")]
    Delete {
        /// Partition path
        path: PathBuf,

        /// Force
        #[arg(short = 'f', long = "force", default_value_t = false)]
        force: bool,
    },
}

// TODO: Implement partition commands
#[allow(unreachable_code, unused_variables)]
impl Command for PartitionCommand {
    fn execute(&self) -> anyhow::Result<(), anyhow::Error> {
        match self {
            PartitionCommand::New {
                filesystem,
                path,
                fat_size,
                size,
                mount,
            } => {
                todo!();
            }
            PartitionCommand::List { path } => {
                if let Some(path) = path {
                    let mut device = Device::new(path.to_owned())?;
                    print_partitions(&mut device)?;
                } else {
                    let devices = utils::list_devices()?;
                    for mut device in devices {
                        print_partitions(&mut device)?;
                    }
                };
            }
            PartitionCommand::Delete { path, force } => {
                todo!();
            }
        }

        Ok(())
    }
}

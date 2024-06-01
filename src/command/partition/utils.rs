use std::path::PathBuf;

use anyhow::Ok;
use libparted::{Device, DeviceIter, Disk};

pub fn list_devices() -> anyhow::Result<DeviceIter<'static>> {
    Ok(Device::devices(true))
}

pub fn list_partitions_paths(disk: Disk) -> anyhow::Result<Vec<PathBuf>> {
    let partitions = disk
        .parts()
        .filter(|p| p.get_path().is_some())
        .map(|p| p.get_path().unwrap().to_owned())
        .collect();

    Ok(partitions)
}

pub fn print_partitions(device: &mut Device) -> anyhow::Result<()> {
    let disk = Disk::new(device)?;

    let partitions = list_partitions_paths(disk)?;

    println!("{}", device.path().to_str().unwrap());
    for partition in partitions {
        println!(" - {}", partition.to_str().unwrap());
    }

    Ok(())
}

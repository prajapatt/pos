mod disks;
mod volumes;

use disks::{Disk, DiskHealth};
use volumes::{Volume, VolumeManager};

#[derive(Clone, Debug, Default)]
pub struct StorageManager {
    disks: Vec<Disk>,
    volumes: VolumeManager,
}

impl StorageManager {
    pub fn add_disk(&mut self, disk: Disk) {
        self.disks.push(disk);
    }

    pub fn mount(&mut self, volume: Volume) {
        self.volumes.mount(volume);
    }
}

fn main() {
    let mut manager = StorageManager::default();
    manager.add_disk(Disk::new("nvme0n1", 1024));
    println!("disks: {}", manager.disks.len());
}

mod discovery;
mod hotplug;

use discovery::{DeviceDescriptor, DeviceDiscovery};
use hotplug::{DeviceEvent, HotplugObserver};

#[derive(Clone, Debug, Default)]
pub struct DeviceManager {
    discovered: Vec<DeviceDescriptor>,
    observers: Vec<HotplugObserver>,
}

impl DeviceManager {
    pub fn register_device(&mut self, device: DeviceDescriptor) {
        self.discovered.push(device);
    }

    pub fn detect(&mut self) -> Vec<DeviceDescriptor> {
        let mut found = DeviceDiscovery::new().scan();
        self.discovered.append(&mut found);
        self.discovered.clone()
    }

    pub fn handle_event(&mut self, event: DeviceEvent) {
        for observer in &mut self.observers {
            observer.notify(event.clone());
        }
    }
}

fn main() {
    let mut manager = DeviceManager::default();
    manager.register_device(DeviceDescriptor::new("cpu", "x86_64") );
    println!("devices: {}", manager.detect().len());
}

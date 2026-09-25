#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceDescriptor {
    pub name: String,
    pub kind: String,
    pub path: String,
    pub online: bool,
}

impl DeviceDescriptor {
    pub fn new(name: &str, kind: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: kind.to_string(),
            path: format!("/dev/{name}"),
            online: true,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DeviceDiscovery {
    devices: Vec<DeviceDescriptor>,
}

impl DeviceDiscovery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scan(&mut self) -> Vec<DeviceDescriptor> {
        let discovered = vec![
            DeviceDescriptor::new("gpu", "display"),
            DeviceDescriptor::new("network", "ethernet"),
            DeviceDescriptor::new("storage", "nvme"),
        ];
        self.devices = discovered.clone();
        discovered
    }
}

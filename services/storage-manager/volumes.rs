#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Volume {
    pub name: String,
    pub mount_point: String,
    pub size_gb: u32,
}

impl Volume {
    pub fn new(name: &str, mount_point: &str, size_gb: u32) -> Self {
        Self {
            name: name.to_string(),
            mount_point: mount_point.to_string(),
            size_gb,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct VolumeManager {
    mounted: Vec<Volume>,
}

impl VolumeManager {
    pub fn mount(&mut self, volume: Volume) {
        self.mounted.push(volume);
    }

    pub fn len(&self) -> usize {
        self.mounted.len()
    }
}

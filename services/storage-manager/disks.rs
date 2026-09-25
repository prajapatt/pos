#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskHealth {
    Healthy,
    Warning,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Disk {
    pub name: String,
    pub total_gb: u32,
    pub health: DiskHealth,
}

impl Disk {
    pub fn new(name: &str, total_gb: u32) -> Self {
        Self {
            name: name.to_string(),
            total_gb,
            health: DiskHealth::Healthy,
        }
    }
}

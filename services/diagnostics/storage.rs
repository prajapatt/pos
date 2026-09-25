#[derive(Clone, Copy, Debug, Default)]
pub struct StorageDiagnostics {
    pub total_gb: u32,
    pub used_gb: u32,
    pub free_gb: u32,
}

impl StorageDiagnostics {
    pub fn sample(&self) -> Self {
        *self
    }
}

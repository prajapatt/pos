#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryDiagnostics {
    pub total_mb: u32,
    pub used_mb: u32,
    pub free_mb: u32,
}

impl MemoryDiagnostics {
    pub fn sample(&self) -> Self {
        *self
    }
}

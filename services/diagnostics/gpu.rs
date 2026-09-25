#[derive(Clone, Copy, Debug, Default)]
pub struct GpuDiagnostics {
    pub utilization_percent: u8,
    pub temperature_c: u16,
    pub memory_used_mb: u32,
}

impl GpuDiagnostics {
    pub fn sample(&self) -> Self {
        *self
    }
}

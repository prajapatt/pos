#[derive(Clone, Copy, Debug, Default)]
pub struct CpuDiagnostics {
    pub load_percent: u8,
    pub temperature_c: u16,
    pub core_count: u8,
}

impl CpuDiagnostics {
    pub fn sample(&self) -> Self {
        *self
    }
}

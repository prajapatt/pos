#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceLimit {
    pub cpu_quota: u8,
    pub memory_mb: u32,
    pub io_weight: u8,
}

#[derive(Clone, Debug, Default)]
pub struct ResourcePolicy {
    limit: ResourceLimit,
}

impl ResourcePolicy {
    pub fn apply(&mut self, limit: ResourceLimit) {
        self.limit = limit;
    }

    pub fn current(&self) -> ResourceLimit {
        self.limit
    }
}

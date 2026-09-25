#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadedImage {
    pub entry: u64,
    pub base: u64,
    pub size: usize,
}

impl LoadedImage {
    pub const fn new(entry: u64, base: u64, size: usize) -> Self {
        Self { entry, base, size }
    }

    pub fn contains(&self, address: u64) -> bool {
        address >= self.base && address < self.base.saturating_add(self.size as u64)
    }
}

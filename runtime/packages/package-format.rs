#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackageHeader {
    pub magic: u32,
    pub version: u16,
    pub payload_size: u32,
}

impl PackageHeader {
    pub const fn new(magic: u32, version: u16, payload_size: u32) -> Self {
        Self { magic, version, payload_size }
    }

    pub fn valid(&self) -> bool {
        self.magic == 0x43554854 && self.version != 0
    }
}

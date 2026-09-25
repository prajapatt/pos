#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SharedRegion { pub base: u64, pub length: u64, pub writable: bool }
impl SharedRegion { pub fn validate(&self) -> Result<(), &'static str> { if self.base & 0xfff != 0 || self.length == 0 || self.length & 0xfff != 0 { Err("shared memory must be page aligned") } else { Ok(()) } } }

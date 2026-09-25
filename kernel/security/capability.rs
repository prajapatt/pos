#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Capability { ReadMemory = 0, WriteMemory = 1, ReadFile = 2, WriteFile = 3, SpawnProcess = 4, ControlDevice = 5, Admin = 63 }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapabilitySet { bits: u64 }
impl CapabilitySet { pub const fn empty() -> Self { Self { bits: 0 } } pub fn grant(&mut self, capability: Capability) { self.bits |= 1u64 << capability as u8; } pub fn revoke(&mut self, capability: Capability) { self.bits &= !(1u64 << capability as u8); } pub const fn allows(&self, capability: Capability) -> bool { self.bits & (1u64 << capability as u8) != 0 } }

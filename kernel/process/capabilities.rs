use crate::security::capability::{Capability, CapabilitySet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessCapabilities { pub effective: CapabilitySet }
impl ProcessCapabilities { pub const fn empty() -> Self { Self { effective: CapabilitySet::empty() } } pub fn grant(&mut self, capability: Capability) { self.effective.grant(capability); } pub const fn allows(&self, capability: Capability) -> bool { self.effective.allows(capability) } }

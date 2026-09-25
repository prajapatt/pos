#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpuState { Discovered, Ready, Failed }

pub struct GpuDevice { pub vendor_id: u16, pub device_id: u16, pub state: GpuState }
impl GpuDevice { pub fn discover(vendor_id: u16, device_id: u16) -> Option<Self> { if vendor_id == 0xffff || device_id == 0xffff { None } else { Some(Self { vendor_id, device_id, state: GpuState::Discovered }) } } pub fn mark_ready(&mut self) { self.state = GpuState::Ready; } }

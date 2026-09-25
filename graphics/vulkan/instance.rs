#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceState { Unavailable, Created }

pub struct VulkanInstance { pub api_major: u32, pub api_minor: u32, pub state: InstanceState }
impl VulkanInstance { pub const fn unavailable() -> Self { Self { api_major: 0, api_minor: 0, state: InstanceState::Unavailable } } pub const fn created(major: u32, minor: u32) -> Self { Self { api_major: major, api_minor: minor, state: InstanceState::Created } } }

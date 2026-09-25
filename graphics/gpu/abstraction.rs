#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Backend { Software, Vulkan }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Device { pub id: u32, pub backend: Backend, pub valid: bool }
impl Device { pub const fn software() -> Self { Self { id: 0, backend: Backend::Software, valid: true } } pub const fn vulkan(id: u32) -> Self { Self { id, backend: Backend::Vulkan, valid: true } } }

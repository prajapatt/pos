#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpuBackend { None, Virtio, Hardware }

pub struct Gpu { backend: GpuBackend }
impl Gpu { pub const fn new(backend: GpuBackend) -> Self { Self { backend } } pub const fn backend(&self) -> GpuBackend { self.backend } }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpuAllocation { pub offset: u64, pub size: u64 }

pub struct GpuMemory { capacity: u64, next: u64 }
impl GpuMemory { pub const fn new(capacity: u64) -> Self { Self { capacity, next: 0 } } pub fn allocate(&mut self, size: u64, alignment: u64) -> Option<GpuAllocation> { if size == 0 || alignment == 0 || !alignment.is_power_of_two() { return None; } let offset = (self.next + alignment - 1) & !(alignment - 1); let end = offset.checked_add(size)?; if end > self.capacity { return None; } self.next = end; Some(GpuAllocation { offset, size }) } }

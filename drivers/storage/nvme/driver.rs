#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueConfig { pub depth: u16, pub submission_base: u64, pub completion_base: u64 }

impl QueueConfig { pub fn validate(&self) -> Result<(), &'static str> { if self.depth < 2 || !self.depth.is_power_of_two() { return Err("NVMe queue depth must be a power of two"); } if self.submission_base & 0x3f != 0 || self.completion_base & 0x3f != 0 { return Err("NVMe queues require 64-byte alignment"); } Ok(()) } }

pub const ADMIN_QUEUE_ATTRIBUTES: u32 = 0x000c;
pub const CONTROLLER_ENABLE: u32 = 1;

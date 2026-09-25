use super::instance::{InstanceState, VulkanInstance};

pub struct VulkanDevice { pub instance: VulkanInstance, pub queue_count: u32 }
impl VulkanDevice { pub fn create(instance: VulkanInstance, queue_count: u32) -> Result<Self, &'static str> { if instance.state != InstanceState::Created || queue_count == 0 { return Err("Vulkan device prerequisites are unavailable"); } Ok(Self { instance, queue_count }) } }

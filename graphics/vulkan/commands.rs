#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VulkanCommand { BeginRenderPass, EndRenderPass, Draw(u32) }

#[derive(Default)]
pub struct VulkanCommandList { commands: Vec<VulkanCommand> }
impl VulkanCommandList { pub fn push(&mut self, command: VulkanCommand) -> Result<(), &'static str> { if let VulkanCommand::Draw(vertices) = command { if vertices == 0 { return Err("draw requires vertices"); } } self.commands.push(command); Ok(()) } pub fn commands(&self) -> &[VulkanCommand] { &self.commands } }

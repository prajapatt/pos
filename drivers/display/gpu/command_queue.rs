pub const QUEUE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpuCommand { pub opcode: u32, pub argument: u64 }

#[derive(Default)]
pub struct CommandQueue { commands: Vec<GpuCommand> }
impl CommandQueue { pub fn push(&mut self, command: GpuCommand) -> Result<(), GpuCommand> { if self.commands.len() >= QUEUE_CAPACITY { return Err(command); } self.commands.push(command); Ok(()) } pub fn pop(&mut self) -> Option<GpuCommand> { if self.commands.is_empty() { None } else { Some(self.commands.remove(0)) } } pub fn len(&self) -> usize { self.commands.len() } }

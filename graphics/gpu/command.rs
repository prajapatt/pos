#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command { Clear(u32), Draw { vertices: u32, first: u32 } }

#[derive(Default)]
pub struct CommandBuffer { commands: Vec<Command>, recording: bool }
impl CommandBuffer { pub fn begin(&mut self) { self.commands.clear(); self.recording = true; } pub fn push(&mut self, command: Command) -> Result<(), &'static str> { if !self.recording { return Err("command buffer is not recording"); } self.commands.push(command); Ok(()) } pub fn end(&mut self) { self.recording = false; } pub fn commands(&self) -> &[Command] { &self.commands } }

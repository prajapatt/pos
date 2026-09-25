use super::framebuffer::Framebuffer;
use crate::gpu::command::{Command, CommandBuffer};

pub struct Renderer { pub framebuffer: Framebuffer, commands: CommandBuffer }
impl Renderer { pub fn new(width: u32, height: u32) -> Result<Self, &'static str> { Ok(Self { framebuffer: Framebuffer::new(width, height)?, commands: CommandBuffer::default() }) } pub fn clear(&mut self, color: u32) -> Result<(), &'static str> { self.commands.begin(); self.commands.push(Command::Clear(color))?; self.commands.end(); self.framebuffer.clear(color); Ok(()) } }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferUsage { Vertex, Index, Uniform, Transfer }

pub struct Buffer { pub usage: BufferUsage, bytes: Vec<u8> }
impl Buffer { pub fn new(usage: BufferUsage, size: usize) -> Result<Self, &'static str> { if size == 0 { return Err("buffer size must be non-zero"); } Ok(Self { usage, bytes: vec![0; size] }) } pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> { if offset.checked_add(data.len()).is_none_or(|end| end > self.bytes.len()) { return Err("buffer write out of bounds"); } self.bytes[offset..offset + data.len()].copy_from_slice(data); Ok(()) } pub fn size(&self) -> usize { self.bytes.len() } }

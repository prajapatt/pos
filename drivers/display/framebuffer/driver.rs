#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Framebuffer { pub address: usize, pub width: u32, pub height: u32, pub stride: u32 }

impl Framebuffer {
	pub const fn new(address: usize, width: u32, height: u32, stride: u32) -> Option<Self> { if address == 0 || width == 0 || height == 0 || stride < width { None } else { Some(Self { address, width, height, stride }) } }
	pub fn pixel_offset(&self, x: u32, y: u32) -> Option<usize> { if x >= self.width || y >= self.height { return None; } Some(((y * self.stride + x) * 4) as usize) }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Framebuffer {
	pub address: u64,
	pub size: usize,
	pub width: u32,
	pub height: u32,
	pub stride: u32,
	pub pixel_format: u32,
}

impl Framebuffer {
	pub const fn is_valid(&self) -> bool {
		self.address != 0 && self.size != 0 && self.width != 0 && self.height != 0 && self.stride >= self.width
	}
}

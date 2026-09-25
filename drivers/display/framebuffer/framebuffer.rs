pub struct PixelBuffer<'a> { pub bytes: &'a mut [u8], pub width: u32, pub height: u32, pub stride: u32 }

impl<'a> PixelBuffer<'a> {
	pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) -> Result<(), &'static str> { if x >= self.width || y >= self.height || self.stride < self.width { return Err("pixel outside framebuffer"); } let offset = ((y * self.stride + x) * 4) as usize; if offset + 4 > self.bytes.len() { return Err("framebuffer is smaller than its geometry"); } self.bytes[offset..offset + 4].copy_from_slice(&color.to_le_bytes()); Ok(()) }
}

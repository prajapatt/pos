#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Screenshot {
	pub width: u32,
	pub height: u32,
	pub stride: u32,
	pub pixels_rgba: Vec<u8>,
}

impl Screenshot {
	pub fn from_rgba(width: u32, height: u32, pixels_rgba: Vec<u8>) -> Result<Self, &'static str> {
		let expected = (width as usize).checked_mul(height as usize).and_then(|v| v.checked_mul(4)).ok_or("image size overflow")?;
		if width == 0 || height == 0 || pixels_rgba.len() != expected { return Err("invalid RGBA buffer"); }
		Ok(Self { width, height, stride: width * 4, pixels_rgba })
	}
}

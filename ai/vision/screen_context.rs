#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScreenContext {
	pub width: u32,
	pub height: u32,
	pub active_application: Option<String>,
	pub focused_element: Option<String>,
}

impl ScreenContext {
	pub fn new(width: u32, height: u32) -> Result<Self, &'static str> {
		if width == 0 || height == 0 { return Err("screen dimensions must be non-zero"); }
		Ok(Self { width, height, active_application: None, focused_element: None })
	}
}

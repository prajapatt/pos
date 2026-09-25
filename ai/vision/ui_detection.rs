use super::screenshot::Screenshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiElement {
	pub role: String,
	pub label: String,
	pub x: u32,
	pub y: u32,
	pub width: u32,
	pub height: u32,
}

pub trait UiDetector {
	fn detect(&self, screenshot: &Screenshot) -> Vec<UiElement>;
}

pub struct EmptyDetector;

impl UiDetector for EmptyDetector {
	fn detect(&self, _screenshot: &Screenshot) -> Vec<UiElement> { Vec::new() }
}

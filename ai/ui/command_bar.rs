#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandBar {
	input: String,
	open: bool,
}

impl CommandBar {
	pub fn open() -> Self {
		Self { input: String::new(), open: true }
	}

	pub fn set_input(&mut self, input: impl Into<String>) {
		self.input = input.into();
	}

	pub fn submit(&mut self) -> Option<String> {
		let input = self.input.trim().to_string();
		if input.is_empty() { return None; }
		self.input.clear();
		Some(input)
	}

	pub fn close(&mut self) { self.open = false; }
	pub fn is_open(&self) -> bool { self.open }
}

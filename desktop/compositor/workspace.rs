use super::windows::Window;

#[derive(Default)]
pub struct Workspace { windows: Vec<Window>, focused: Option<u64> }

impl Workspace {
	pub fn add(&mut self, window: Window) { self.windows.push(window); }
	pub fn focus(&mut self, id: u64) -> bool {
		if !self.windows.iter().any(|window| window.id == id) { return false; }
		for window in &mut self.windows { window.focused = window.id == id; }
		self.focused = Some(id); true
	}
	pub fn windows(&self) -> &[Window] { &self.windows }
	pub fn focused(&self) -> Option<u64> { self.focused }
}

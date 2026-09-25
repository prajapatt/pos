use super::surfaces::{Rect, Surface};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Window { pub id: u64, pub title: String, pub surface: Surface, pub focused: bool }

impl Window {
	pub fn new(id: u64, title: impl Into<String>, bounds: Rect) -> Self { Self { id, title: title.into(), surface: Surface::new(id, bounds), focused: false } }
}

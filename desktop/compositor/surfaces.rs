#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rect { pub x: i32, pub y: i32, pub width: u32, pub height: u32 }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Surface { pub id: u64, pub bounds: Rect, pub visible: bool }

impl Surface {
	pub fn new(id: u64, bounds: Rect) -> Self { Self { id, bounds, visible: true } }
}

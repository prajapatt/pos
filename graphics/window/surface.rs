#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceState { Unbound, Bound }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Surface { pub width: u32, pub height: u32, pub state: SurfaceState }
impl Surface { pub fn new(width: u32, height: u32) -> Result<Self, &'static str> { if width == 0 || height == 0 { return Err("surface dimensions are zero"); } Ok(Self { width, height, state: SurfaceState::Unbound }) } pub fn bind(&mut self) { self.state = SurfaceState::Bound; } }

use super::surface::{Surface, SurfaceState};

pub struct Swapchain { pub surface: Surface, pub image_count: u32, current: u32 }
impl Swapchain { pub fn create(surface: Surface, image_count: u32) -> Result<Self, &'static str> { if surface.state != SurfaceState::Bound || !(2..=8).contains(&image_count) { return Err("invalid swapchain prerequisites"); } Ok(Self { surface, image_count, current: 0 }) } pub fn acquire_next(&mut self) -> u32 { let index = self.current; self.current = (self.current + 1) % self.image_count; index } }

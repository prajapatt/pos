#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Animation { pub start: i32, pub end: i32, pub duration_ms: u32 }
impl Animation { pub fn value_at(&self, elapsed_ms: u32) -> i32 { if self.duration_ms == 0 || elapsed_ms >= self.duration_ms { return self.end; } self.start + ((self.end - self.start) * elapsed_ms as i32 / self.duration_ms as i32) } }

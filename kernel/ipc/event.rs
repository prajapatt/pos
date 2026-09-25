pub struct Event { sequence: u64, signaled: bool }
impl Event { pub const fn new() -> Self { Self { sequence: 0, signaled: false } } pub fn signal(&mut self) { self.sequence = self.sequence.wrapping_add(1); self.signaled = true; } pub fn consume(&mut self) -> bool { let was = self.signaled; self.signaled = false; was } pub const fn sequence(&self) -> u64 { self.sequence } }

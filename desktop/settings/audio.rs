#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioSettings { pub volume_percent: u8, pub muted: bool }
impl Default for AudioSettings { fn default() -> Self { Self { volume_percent: 100, muted: false } } }
impl AudioSettings { pub fn set_volume(&mut self, value: u8) { self.volume_percent = value.min(100); } }

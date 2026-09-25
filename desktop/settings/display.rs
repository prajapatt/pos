#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplaySettings { pub width: u32, pub height: u32, pub brightness_percent: u8 }
impl Default for DisplaySettings { fn default() -> Self { Self { width: 1024, height: 768, brightness_percent: 100 } } }
impl DisplaySettings { pub fn set_brightness(&mut self, value: u8) { self.brightness_percent = value.min(100); } }

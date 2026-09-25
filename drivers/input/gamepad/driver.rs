#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GamepadReport { pub buttons: u16, pub x: i8, pub y: i8, pub rx: i8, pub ry: i8 }

pub fn decode_report(bytes: &[u8]) -> Option<GamepadReport> { if bytes.len() < 6 { return None; } Some(GamepadReport { buttons: u16::from_le_bytes([bytes[0], bytes[1]]), x: bytes[2] as i8, y: bytes[3] as i8, rx: bytes[4] as i8, ry: bytes[5] as i8 }) }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GamepadState { pub buttons: u16, pub left_x: i16, pub left_y: i16, pub right_x: i16, pub right_y: i16 }

pub fn decode_report(bytes: &[u8]) -> Option<GamepadState> { if bytes.len() < 10 { return None; } Some(GamepadState { buttons: u16::from_le_bytes([bytes[0], bytes[1]]), left_x: i16::from_le_bytes([bytes[2], bytes[3]]), left_y: i16::from_le_bytes([bytes[4], bytes[5]]), right_x: i16::from_le_bytes([bytes[6], bytes[7]]), right_y: i16::from_le_bytes([bytes[8], bytes[9]]) }) }

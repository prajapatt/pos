#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TouchPoint { pub id: u8, pub x: u16, pub y: u16, pub pressure: u16 }

pub fn decode_point(bytes: &[u8]) -> Option<TouchPoint> { if bytes.len() < 7 { return None; } Some(TouchPoint { id: bytes[0], x: u16::from_le_bytes([bytes[1], bytes[2]]), y: u16::from_le_bytes([bytes[3], bytes[4]]), pressure: u16::from_le_bytes([bytes[5], bytes[6]]) }) }

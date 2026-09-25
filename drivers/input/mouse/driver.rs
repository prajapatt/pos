#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MousePacket { pub buttons: u8, pub dx: i16, pub dy: i16 }

pub fn decode_packet(bytes: [u8; 3]) -> Option<MousePacket> { if bytes[0] & 0x08 == 0 { return None; } let dx = if bytes[0] & 0x10 != 0 { i16::from(bytes[1]) - 256 } else { i16::from(bytes[1]) }; let dy = if bytes[0] & 0x20 != 0 { i16::from(bytes[2]) - 256 } else { i16::from(bytes[2]) }; Some(MousePacket { buttons: bytes[0] & 7, dx, dy: -dy }) }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MessageHeader { pub sender: u64, pub kind: u16, pub length: u16 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Message { pub header: MessageHeader, pub payload: [u8; 64] }
impl Message { pub fn new(sender: u64, kind: u16, payload: &[u8]) -> Result<Self, &'static str> { if payload.len() > 64 { return Err("IPC payload too large"); } let mut bytes = [0; 64]; bytes[..payload.len()].copy_from_slice(payload); Ok(Self { header: MessageHeader { sender, kind, length: payload.len() as u16 }, payload: bytes }) } }

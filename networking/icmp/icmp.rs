#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IcmpHeader {
    pub kind: u8,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence: u16,
}

impl IcmpHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 { return None; }
        Some(Self {
            kind: bytes[0],
            code: bytes[1],
            checksum: u16::from_be_bytes([bytes[2], bytes[3]]),
            identifier: u16::from_be_bytes([bytes[4], bytes[5]]),
            sequence: u16::from_be_bytes([bytes[6], bytes[7]]),
        })
    }

    pub fn echo_request(identifier: u16, sequence: u16) -> Self {
        Self { kind: 8, code: 0, checksum: 0, identifier, sequence }
    }
}

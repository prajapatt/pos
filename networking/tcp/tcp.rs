#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TcpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub acknowledgment_number: u32,
    pub data_offset: u8,
    pub flags: u8,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
}

impl TcpHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 { return None; }
        let offset = ((bytes[12] >> 4) & 0x0f) as usize;
        if bytes.len() < offset * 4 { return None; }
        Some(Self {
            source_port: u16::from_be_bytes([bytes[0], bytes[1]]),
            destination_port: u16::from_be_bytes([bytes[2], bytes[3]]),
            sequence_number: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            acknowledgment_number: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            data_offset: (bytes[12] >> 4) * 0x10 + (bytes[12] & 0x0f),
            flags: bytes[13],
            window_size: u16::from_be_bytes([bytes[14], bytes[15]]),
            checksum: u16::from_be_bytes([bytes[16], bytes[17]]),
            urgent_pointer: u16::from_be_bytes([bytes[18], bytes[19]]),
        })
    }

    pub fn is_syn(&self) -> bool { self.flags & 0x02 != 0 }
    pub fn is_ack(&self) -> bool { self.flags & 0x10 != 0 }
    pub fn is_fin(&self) -> bool { self.flags & 0x01 != 0 }
}

#[cfg(test)]
mod tests {
    use super::TcpHeader;

    #[test]
    fn tcp_header_detects_flags() {
        let mut bytes = [0u8; 20];
        bytes[0..2].copy_from_slice(&0x5000u16.to_be_bytes());
        bytes[2..4].copy_from_slice(&0x5001u16.to_be_bytes());
        bytes[12] = 0x50;
        bytes[13] = 0x12;

        let header = TcpHeader::parse(&bytes).unwrap();
        assert!(header.is_syn());
        assert!(header.is_ack());
    }
}

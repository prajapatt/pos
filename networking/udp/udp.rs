#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UdpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UdpHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 { return None; }
        Some(Self {
            source_port: u16::from_be_bytes([bytes[0], bytes[1]]),
            destination_port: u16::from_be_bytes([bytes[2], bytes[3]]),
            length: u16::from_be_bytes([bytes[4], bytes[5]]),
            checksum: u16::from_be_bytes([bytes[6], bytes[7]]),
        })
    }

    pub fn payload_offset() -> usize { 8 }
}

#[cfg(test)]
mod tests {
    use super::UdpHeader;

    #[test]
    fn udp_header_parses_ports() {
        let bytes = [0x00, 0x35, 0x00, 0x36, 0x00, 0x08, 0x00, 0x00];
        let header = UdpHeader::parse(&bytes).unwrap();
        assert_eq!(header.source_port, 53);
        assert_eq!(header.destination_port, 54);
        assert_eq!(header.length, 8);
    }
}

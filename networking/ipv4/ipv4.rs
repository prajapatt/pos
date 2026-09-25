#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ipv4Address([u8; 4]);

impl Ipv4Address {
    pub fn new(octets: [u8; 4]) -> Self { Self(octets) }
    pub fn bytes(&self) -> [u8; 4] { self.0 }
    pub fn is_loopback(&self) -> bool { self.0[0] == 127 }
    pub fn is_multicast(&self) -> bool { self.0[0] >= 224 && self.0[0] <= 239 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ipv4Header {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source: Ipv4Address,
    pub destination: Ipv4Address,
}

impl Ipv4Header {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 { return None; }
        let version = bytes[0] >> 4;
        let ihl = (bytes[0] & 0x0f) * 4;
        if ihl < 20 || bytes.len() < ihl as usize { return None; }
        let total_length = u16::from_be_bytes([bytes[2], bytes[3]]);
        Some(Self {
            version,
            ihl,
            dscp: bytes[1],
            total_length,
            identification: u16::from_be_bytes([bytes[4], bytes[5]]),
            flags_fragment_offset: u16::from_be_bytes([bytes[6], bytes[7]]),
            ttl: bytes[8],
            protocol: bytes[9],
            header_checksum: u16::from_be_bytes([bytes[10], bytes[11]]),
            source: Ipv4Address::new([bytes[12], bytes[13], bytes[14], bytes[15]]),
            destination: Ipv4Address::new([bytes[16], bytes[17], bytes[18], bytes[19]]),
        })
    }

    pub fn checksum(bytes: &[u8]) -> u16 {
        let mut sum = 0u32;
        for chunk in bytes.chunks_exact(2) {
            sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        }
        while sum >> 16 != 0 { sum = (sum & 0xffff) + (sum >> 16); }
        (!sum as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::Ipv4Header;

    #[test]
    fn ipv4_header_parses_basic_fields() {
        let mut bytes = [0u8; 20];
        bytes[0] = 0x45;
        bytes[2] = 0x00;
        bytes[3] = 20;
        bytes[8] = 64;
        bytes[9] = 6;
        bytes[12..16].copy_from_slice(&[10, 0, 0, 1]);
        bytes[16..20].copy_from_slice(&[10, 0, 0, 2]);

        let header = Ipv4Header::parse(&bytes).unwrap();
        assert_eq!(header.version, 4);
        assert_eq!(header.total_length, 20);
        assert_eq!(header.protocol, 6);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TlsVersion {
    V1_2,
    V1_3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TlsRecordHeader {
    pub content_type: u8,
    pub version: TlsVersion,
    pub length: u16,
}

impl TlsRecordHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 5 { return None; }
        let content_type = bytes[0];
        let version = match u16::from_be_bytes([bytes[1], bytes[2]]) {
            0x0303 => TlsVersion::V1_2,
            0x0304 => TlsVersion::V1_3,
            _ => return None,
        };
        let length = u16::from_be_bytes([bytes[3], bytes[4]]);
        Some(Self { content_type, version, length })
    }

    pub fn is_handshake(&self) -> bool { self.content_type == 22 }
}

#[cfg(test)]
mod tests {
    use super::TlsRecordHeader;

    #[test]
    fn tls_header_parses_known_versions() {
        let bytes = [0x16, 0x03, 0x03, 0x00, 0x20];
        let header = TlsRecordHeader::parse(&bytes).unwrap();
        assert!(header.is_handshake());
    }
}

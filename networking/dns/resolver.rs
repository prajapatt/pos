#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DnsQuestion {
    pub qname: &'static str,
    pub qtype: u16,
    pub qclass: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

impl DnsHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 12 { return None; }
        Some(Self {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            flags: u16::from_be_bytes([bytes[2], bytes[3]]),
            question_count: u16::from_be_bytes([bytes[4], bytes[5]]),
            answer_count: u16::from_be_bytes([bytes[6], bytes[7]]),
            authority_count: u16::from_be_bytes([bytes[8], bytes[9]]),
            additional_count: u16::from_be_bytes([bytes[10], bytes[11]]),
        })
    }

    pub fn is_response(&self) -> bool { self.flags & 0x8000 != 0 }
}

pub fn encode_name(name: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for label in name.split('.') {
        let bytes = label.as_bytes();
        out.push(bytes.len() as u8);
        out.extend_from_slice(bytes);
    }
    out.push(0);
    out
}

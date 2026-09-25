#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ipv6Address([u16; 8]);

impl Ipv6Address {
    pub fn new(words: [u16; 8]) -> Self { Self(words) }
    pub fn words(&self) -> [u16; 8] { self.0 }
    pub fn is_loopback(&self) -> bool { self.0 == [0, 0, 0, 0, 0, 0, 0, 1] }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ipv6Header {
    pub version: u8,
    pub traffic_class: u8,
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub source: Ipv6Address,
    pub destination: Ipv6Address,
}

impl Ipv6Header {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 40 { return None; }
        let version = (bytes[0] >> 4) as u8;
        let traffic_class = ((bytes[0] & 0x0f) << 4) | ((bytes[1] >> 4) & 0x0f);
        let flow_label = ((bytes[1] & 0x0f) as u32) << 16 | (u32::from(bytes[2]) << 8) | u32::from(bytes[3]);
        let payload_length = u16::from_be_bytes([bytes[4], bytes[5]]);
        let next_header = bytes[6];
        let hop_limit = bytes[7];
        let source = Ipv6Address::new([
            u16::from_be_bytes([bytes[8], bytes[9]]), u16::from_be_bytes([bytes[10], bytes[11]]),
            u16::from_be_bytes([bytes[12], bytes[13]]), u16::from_be_bytes([bytes[14], bytes[15]]),
            u16::from_be_bytes([bytes[16], bytes[17]]), u16::from_be_bytes([bytes[18], bytes[19]]),
            u16::from_be_bytes([bytes[20], bytes[21]]), u16::from_be_bytes([bytes[22], bytes[23]]),
        ]);
        let destination = Ipv6Address::new([
            u16::from_be_bytes([bytes[24], bytes[25]]), u16::from_be_bytes([bytes[26], bytes[27]]),
            u16::from_be_bytes([bytes[28], bytes[29]]), u16::from_be_bytes([bytes[30], bytes[31]]),
            u16::from_be_bytes([bytes[32], bytes[33]]), u16::from_be_bytes([bytes[34], bytes[35]]),
            u16::from_be_bytes([bytes[36], bytes[37]]), u16::from_be_bytes([bytes[38], bytes[39]]),
        ]);
        Some(Self { version, traffic_class, flow_label, payload_length, next_header, hop_limit, source, destination })
    }
}

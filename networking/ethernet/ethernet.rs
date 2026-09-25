#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EthernetAddress([u8; 6]);

impl EthernetAddress {
    pub fn new(bytes: [u8; 6]) -> Self { Self(bytes) }
    pub fn bytes(&self) -> [u8; 6] { self.0 }
    pub fn is_broadcast(&self) -> bool { self.0.iter().all(|b| *b == 0xff) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EthernetFrame<'a> {
    pub destination: EthernetAddress,
    pub source: EthernetAddress,
    pub ether_type: u16,
    pub payload: &'a [u8],
}

impl<'a> EthernetFrame<'a> {
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() < 14 { return None; }
        let destination = EthernetAddress::new([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]]);
        let source = EthernetAddress::new([bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11]]);
        let ether_type = u16::from_be_bytes([bytes[12], bytes[13]]);
        let payload = &bytes[14..];
        Some(Self { destination, source, ether_type, payload })
    }

    pub fn serialize(&self, out: &mut [u8]) -> Result<(), &'static str> {
        if out.len() < 14 + self.payload.len() { return Err("ethernet frame too small"); }
        out[0..6].copy_from_slice(&self.destination.bytes());
        out[6..12].copy_from_slice(&self.source.bytes());
        out[12..14].copy_from_slice(&self.ether_type.to_be_bytes());
        out[14..14 + self.payload.len()].copy_from_slice(self.payload);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::EthernetFrame;

    #[test]
    fn ethernet_frame_parses_and_serializes() {
        let mut bytes = [0u8; 14 + 4];
        bytes[0..6].copy_from_slice(&[1, 2, 3, 4, 5, 6]);
        bytes[6..12].copy_from_slice(&[6, 5, 4, 3, 2, 1]);
        bytes[12..14].copy_from_slice(&0x0800u16.to_be_bytes());
        bytes[14..18].copy_from_slice(&[9, 8, 7, 6]);

        let frame = EthernetFrame::parse(&bytes).unwrap();
        assert_eq!(frame.ether_type, 0x0800);
        assert_eq!(frame.payload, &[9, 8, 7, 6]);
    }
}

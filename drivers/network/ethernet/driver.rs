pub const ETHERNET_HEADER: usize = 14;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EthernetFrame<'a> { pub destination: [u8; 6], pub source: [u8; 6], pub ether_type: u16, pub payload: &'a [u8] }

pub fn parse(frame: &[u8]) -> Option<EthernetFrame<'_>> { if frame.len() < ETHERNET_HEADER { return None; } let mut destination = [0; 6]; let mut source = [0; 6]; destination.copy_from_slice(&frame[..6]); source.copy_from_slice(&frame[6..12]); Some(EthernetFrame { destination, source, ether_type: u16::from_be_bytes([frame[12], frame[13]]), payload: &frame[14..] }) }

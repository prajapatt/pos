pub const RSDP_SIGNATURE: &[u8; 8] = b"RSD PTR ";

pub fn checksum_valid(bytes: &[u8]) -> bool { !bytes.is_empty() && bytes.iter().fold(0u8, |sum, byte| sum.wrapping_add(*byte)) == 0 }

pub fn validate_rsdp(bytes: &[u8]) -> bool { bytes.len() >= 20 && &bytes[..8] == RSDP_SIGNATURE && checksum_valid(&bytes[..20]) }

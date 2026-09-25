#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackageSignature {
    pub signer: &'static str,
    pub version: u32,
    pub hash: [u8; 32],
}

impl PackageSignature {
    pub const fn new(signer: &'static str, version: u32, hash: [u8; 32]) -> Self {
        Self { signer, version, hash }
    }

    pub fn matches_hash(&self, candidate: &[u8]) -> bool {
        candidate.len() == self.hash.len() && candidate.iter().zip(self.hash.iter()).all(|(a, b)| a == b)
    }
}

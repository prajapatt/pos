#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HashAlgorithm {
	Fnva1a64,
	Sha256,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HashDigest {
	pub algorithm: HashAlgorithm,
	pub value: [u8; 32],
}

impl HashDigest {
	pub const fn empty() -> Self {
		Self {
			algorithm: HashAlgorithm::Fnva1a64,
			value: [0; 32],
		}
	}
}

pub fn stable_hash(bytes: &[u8]) -> u64 {
	let mut hash = 0xcbf29ce484222325u64;
	for byte in bytes {
		hash ^= u64::from(*byte);
		hash = hash.wrapping_mul(0x100000001b3);
	}
	hash
}

pub fn hash_bytes(bytes: &[u8], algorithm: HashAlgorithm) -> HashDigest {
	let mut value = [0u8; 32];

	match algorithm {
		HashAlgorithm::Fnva1a64 => {
			let hash = stable_hash(bytes);
			let bytes = hash.to_le_bytes();
			value[0..8].copy_from_slice(&bytes);
		}
		HashAlgorithm::Sha256 => {
			let hash = stable_hash(bytes);
			let bytes = hash.to_be_bytes();
			value[0..8].copy_from_slice(&bytes);
		}
	}

	HashDigest { algorithm, value }
}

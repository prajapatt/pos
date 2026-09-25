#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyPurpose {
	Encryption,
	Signing,
	Authentication,
	DeviceIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyId {
	pub value: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SymmetricKey {
	pub id: KeyId,
	pub purpose: KeyPurpose,
	pub material: [u8; 32],
	pub active: bool,
}

impl SymmetricKey {
	pub const fn new(id: u64, purpose: KeyPurpose, material: [u8; 32]) -> Self {
		Self {
			id: KeyId { value: id },
			purpose,
			material,
			active: true,
		}
	}

	pub fn rotate(&mut self, next_material: [u8; 32]) {
		self.material = next_material;
		self.active = true;
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicKey {
	pub id: KeyId,
	pub purpose: KeyPurpose,
	pub modulus: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivateKey {
	pub id: KeyId,
	pub purpose: KeyPurpose,
	pub modulus: [u8; 32],
	pub secret: [u8; 32],
}

impl PrivateKey {
	pub const fn new(id: u64, purpose: KeyPurpose, modulus: [u8; 32], secret: [u8; 32]) -> Self {
		Self {
			id: KeyId { value: id },
			purpose,
			modulus,
			secret,
		}
	}
}

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialKind {
	Password,
	Token,
	Certificate,
	Biometric,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credential {
	pub owner: u32,
	pub kind: CredentialKind,
	pub secret_hash: [u8; 16],
	pub valid: bool,
}

impl Credential {
	pub const fn new(owner: u32, kind: CredentialKind, secret_hash: [u8; 16]) -> Self {
		Self {
			owner,
			kind,
			secret_hash,
			valid: true,
		}
	}

	pub fn revoke(&mut self) {
		self.valid = false;
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthToken {
	pub user_id: u32,
	pub nonce: u64,
	pub expires_at: u64,
}

impl AuthToken {
	pub const fn new(user_id: u32, nonce: u64, expires_at: u64) -> Self {
		Self {
			user_id,
			nonce,
			expires_at,
		}
	}

	pub const fn is_valid(&self, now: u64) -> bool {
		self.expires_at > now && self.user_id != 0
	}
}

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserRole {
	Guest,
	Standard,
	Operator,
	Administrator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserAccount {
	pub id: u32,
	pub name: [u8; 32],
	pub role: UserRole,
	pub is_locked: bool,
}

impl UserAccount {
	pub const fn new(id: u32, name: [u8; 32], role: UserRole) -> Self {
		Self {
			id,
			name,
			role,
			is_locked: false,
		}
	}

	pub fn lock(&mut self) {
		self.is_locked = true;
	}

	pub fn unlock(&mut self) {
		self.is_locked = false;
	}

	pub fn can_administrate(&self) -> bool {
		matches!(self.role, UserRole::Administrator)
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Principal {
	pub user: UserAccount,
	pub session_id: u64,
}

impl Principal {
	pub const fn new(user: UserAccount, session_id: u64) -> Self {
		Self { user, session_id }
	}

	pub fn is_active(&self) -> bool {
		!self.user.is_locked && self.session_id != 0
	}
}

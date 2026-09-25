#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GroupId {
	pub value: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GroupMembership {
	pub user_id: u32,
	pub group: GroupId,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IdentityGroup {
	pub id: GroupId,
	pub name: [u8; 32],
	pub role: u32,
}

impl IdentityGroup {
	pub const fn new(id: u32, name: [u8; 32], role: u32) -> Self {
		Self {
			id: GroupId { value: id },
			name,
			role,
		}
	}

	pub fn matches_role(&self, required_role: u32) -> bool {
		self.role >= required_role
	}
}

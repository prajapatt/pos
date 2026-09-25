#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscallClass {
	File,
	Process,
	Network,
	Device,
	Memory,
	Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyscallRule {
	pub name: [u8; 16],
	pub class: SyscallClass,
	pub allowed: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SyscallFilter {
	pub rules: [SyscallRule; 32],
	pub len: usize,
}

impl SyscallFilter {
	pub const fn new() -> Self {
		Self {
			rules: [SyscallRule {
				name: [0; 16],
				class: SyscallClass::Unknown,
				allowed: false,
			}; 32],
			len: 0,
		}
	}

	pub fn add_rule(&mut self, name: &[u8], class: SyscallClass, allowed: bool) {
		if self.len >= self.rules.len() {
			return;
		}

		let mut fixed = [0u8; 16];
		let copy_len = name.len().min(fixed.len());
		fixed[..copy_len].copy_from_slice(&name[..copy_len]);

		self.rules[self.len] = SyscallRule {
			name: fixed,
			class,
			allowed,
		};
		self.len += 1;
	}

	pub fn is_allowed(&self, name: &[u8], class: SyscallClass) -> bool {
		self.rules.iter().take(self.len).any(|rule| {
			let name_matches = rule.name.iter().zip(name.iter()).all(|(a, b)| a == b);
			name_matches && rule.class == class && rule.allowed
		})
	}
}

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileAccessMode {
	Read,
	Write,
	Execute,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PathRule {
	pub prefix: [u8; 48],
	pub allow: bool,
	pub mode: FileAccessMode,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FilesystemPolicy {
	pub allowed_roots: [[u8; 48]; 8],
	pub read_only: bool,
	pub max_files: u32,
}

impl FilesystemPolicy {
	pub fn allow_path(&self, path: &[u8], mode: FileAccessMode) -> bool {
		let matches_prefix = self.allowed_roots.iter().any(|root| {
			let root_len = root.iter().position(|byte| *byte == 0).unwrap_or(root.len());
			path.len() >= root_len && &path[..root_len] == &root[..root_len]
		});

		if !matches_prefix {
			return false;
		}

		if self.read_only && mode == FileAccessMode::Write {
			return false;
		}

		true
	}
}

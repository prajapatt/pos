use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FilesystemError {
	AbsolutePath,
	ParentTraversal,
	OutsideRoot,
	InvalidPath,
	Io(String),
}

pub struct FilesystemTool {
	root: PathBuf,
}

impl FilesystemTool {
	pub fn new(root: impl Into<PathBuf>) -> Result<Self, FilesystemError> {
		let root = root.into();
		let root = root
			.canonicalize()
			.map_err(|error| FilesystemError::Io(error.to_string()))?;
		if !root.is_dir() {
			return Err(FilesystemError::InvalidPath);
		}
		Ok(Self { root })
	}

	pub fn root(&self) -> &Path {
		&self.root
	}

	pub fn read_text(&self, path: &str) -> Result<String, FilesystemError> {
		let path = self.resolve_existing(path)?;
		fs::read_to_string(path).map_err(|error| FilesystemError::Io(error.to_string()))
	}

	pub fn create_directory(&self, path: &str) -> Result<String, FilesystemError> {
		let relative = validate_relative(path)?;
		let target = self.root.join(relative);
		if target.exists() {
			return Err(FilesystemError::Io("path already exists".into()));
		}
		fs::create_dir_all(&target).map_err(|error| FilesystemError::Io(error.to_string()))?;
		let verified = target
			.canonicalize()
			.map_err(|error| FilesystemError::Io(error.to_string()))?;
		if !verified.starts_with(&self.root) || !verified.is_dir() {
			return Err(FilesystemError::OutsideRoot);
		}
		Ok(verified.display().to_string())
	}

	fn resolve_existing(&self, path: &str) -> Result<PathBuf, FilesystemError> {
		let relative = validate_relative(path)?;
		let resolved = self
			.root
			.join(relative)
			.canonicalize()
			.map_err(|error| FilesystemError::Io(error.to_string()))?;
		if !resolved.starts_with(&self.root) {
			return Err(FilesystemError::OutsideRoot);
		}
		Ok(resolved)
	}
}

fn validate_relative(path: &str) -> Result<PathBuf, FilesystemError> {
	let path = Path::new(path);
	if path.as_os_str().is_empty() || path.is_absolute() {
		return Err(FilesystemError::AbsolutePath);
	}
	for component in path.components() {
		if matches!(component, Component::ParentDir) {
			return Err(FilesystemError::ParentTraversal);
		}
		if matches!(component, Component::Prefix(_)) {
			return Err(FilesystemError::AbsolutePath);
		}
	}
	Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
	use super::{FilesystemError, FilesystemTool};

	#[test]
	fn rejects_paths_outside_root() {
		let root = std::env::temp_dir();
		let tool = FilesystemTool::new(&root).unwrap();

		assert_eq!(tool.read_text("../outside"), Err(FilesystemError::ParentTraversal));
		assert_eq!(tool.read_text("C:\\outside"), Err(FilesystemError::AbsolutePath));
	}

	#[test]
	fn creates_and_reads_a_directory_child() {
		let root = std::env::temp_dir().join(format!("pos-fs-{}", std::process::id()));
		std::fs::create_dir_all(&root).unwrap();
		let tool = FilesystemTool::new(&root).unwrap();
		let child = root.join("real-child");
		assert!(tool.create_directory("real-child").is_ok());
		assert!(child.is_dir());
		std::fs::remove_dir_all(root).unwrap();
 	}
}

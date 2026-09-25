#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelImage {
	pub base: u64,
	pub size: u64,
	pub entry: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoaderError {
	EmptyImage,
	UnalignedBase,
	EntryOutsideImage,
}

impl KernelImage {
	pub fn validate(self) -> Result<Self, LoaderError> {
		if self.size == 0 { return Err(LoaderError::EmptyImage); }
		if self.base & 0xfff != 0 { return Err(LoaderError::UnalignedBase); }
		let end = self.base.checked_add(self.size).ok_or(LoaderError::EntryOutsideImage)?;
		if self.entry < self.base || self.entry >= end { return Err(LoaderError::EntryOutsideImage); }
		Ok(self)
	}
}

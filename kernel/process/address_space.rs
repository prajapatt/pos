use crate::memory::page::{PageAddress, PageFlags};
use crate::memory::r#virtual::{Mapping, MappingTable, VirtualMemoryError};

pub struct AddressSpace<const MAX_MAPPINGS: usize> { pub root: PageAddress, mappings: MappingTable<MAX_MAPPINGS> }
impl<const MAX_MAPPINGS: usize> AddressSpace<MAX_MAPPINGS> {
	pub const fn new(root: PageAddress) -> Self { Self { root, mappings: MappingTable::new() } }
	pub fn map(&mut self, virtual_page: PageAddress, physical_page: PageAddress, flags: PageFlags) -> Result<(), VirtualMemoryError> {
		self.mappings.map(virtual_page, physical_page, flags)
	}
	pub fn unmap(&mut self, virtual_page: PageAddress) -> bool {
		self.mappings.unmap(virtual_page).is_ok()
	}
	pub fn translate(&self, virtual_page: PageAddress) -> Option<PageAddress> {
		self.mappings.get(virtual_page).map(|mapping| mapping.physical_page)
	}
	pub fn has_mapping(&self, virtual_page: PageAddress) -> bool {
		self.mappings.get(virtual_page).is_some()
	}
}

impl<const MAX_MAPPINGS: usize> Default for AddressSpace<MAX_MAPPINGS> {
	fn default() -> Self {
		Self::new(PageAddress::new(0).expect("page 0 must be aligned"))
	}
}

#[cfg(test)]
mod tests {
	use super::AddressSpace;
	use crate::memory::page::{PageAddress, PageFlags};

	#[test]
	fn address_space_tracks_page_mappings() {
		let mut space = AddressSpace::<2>::new(PageAddress::new(0).unwrap());
		let virtual_page = PageAddress::new(0x1000).unwrap();
		let physical_page = PageAddress::new(0x2000).unwrap();

		assert!(space.map(virtual_page, physical_page, PageFlags::USER).is_ok());
		assert_eq!(space.translate(virtual_page), Some(physical_page));
		assert!(space.unmap(virtual_page));
		assert!(!space.has_mapping(virtual_page));
	}
} 

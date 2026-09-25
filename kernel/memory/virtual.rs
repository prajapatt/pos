use super::page::{PageAddress, PageFlags};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Mapping {
    pub virtual_page: PageAddress,
    pub physical_page: PageAddress,
    pub flags: PageFlags,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VirtualMemoryError {
    TableFull,
    PageAlreadyMapped,
    PageNotMapped,
    InvalidAddress,
}

pub struct MappingTable<const MAX: usize> {
    mappings: [Option<Mapping>; MAX],
    count: usize,
}

impl<const MAX: usize> MappingTable<MAX> {
    pub const fn new() -> Self {
        Self {
            mappings: [None; MAX],
            count: 0,
        }
    }

    pub fn map(&mut self, virtual_page: PageAddress, physical_page: PageAddress, flags: PageFlags) -> Result<(), VirtualMemoryError> {
        if self.find(virtual_page).is_some() {
            return Err(VirtualMemoryError::PageAlreadyMapped);
        }
        if self.count == MAX {
            return Err(VirtualMemoryError::TableFull);
        }
        self.mappings[self.count] = Some(Mapping {
            virtual_page,
            physical_page,
            flags: flags.or(PageFlags::PRESENT),
        });
        self.count += 1;
        Ok(())
    }

    pub fn unmap(&mut self, virtual_page: PageAddress) -> Result<Mapping, VirtualMemoryError> {
        let index = self.find(virtual_page).ok_or(VirtualMemoryError::PageNotMapped)?;
        let removed = self.mappings[index].unwrap();
        self.mappings[index] = self.mappings[self.count - 1];
        self.mappings[self.count - 1] = None;
        self.count -= 1;
        Ok(removed)
    }

    pub fn find(&self, virtual_page: PageAddress) -> Option<usize> {
        self.mappings[..self.count]
            .iter()
            .position(|mapping| mapping.map(|item| item.virtual_page) == Some(virtual_page))
    }

    pub fn get(&self, virtual_page: PageAddress) -> Option<Mapping> {
        self.find(virtual_page)
            .and_then(|index| self.mappings[index])
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

pub struct VirtualAddressSpace<const MAX: usize> {
    mappings: MappingTable<MAX>,
}

impl<const MAX: usize> VirtualAddressSpace<MAX> {
    pub const fn new() -> Self {
        Self { mappings: MappingTable::new() }
    }

    pub fn map_page(
        &mut self,
        virtual_page: PageAddress,
        physical_page: PageAddress,
        flags: PageFlags,
    ) -> Result<(), VirtualMemoryError> {
        self.mappings.map(virtual_page, physical_page, flags)
    }

    pub fn unmap_page(&mut self, virtual_page: PageAddress) -> Result<Mapping, VirtualMemoryError> {
        self.mappings.unmap(virtual_page)
    }

    pub fn translate(&self, virtual_page: PageAddress) -> Option<PageAddress> {
        self.mappings.get(virtual_page).map(|mapping| mapping.physical_page)
    }

    pub fn has_mapping(&self, virtual_page: PageAddress) -> bool {
        self.mappings.get(virtual_page).is_some()
    }
}

impl<const MAX: usize> Default for VirtualAddressSpace<MAX> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{MappingTable, VirtualAddressSpace, VirtualMemoryError};
    use crate::memory::page::{PageAddress, PageFlags};

    #[test]
    fn mapping_table_maps_and_translates_pages() {
        let mut table = MappingTable::<4>::new();
        let virtual_page = PageAddress::new(0x1000).unwrap();
        let physical_page = PageAddress::new(0x2000).unwrap();

        table.map(virtual_page, physical_page, PageFlags::WRITABLE).unwrap();
        assert_eq!(table.get(virtual_page), Some(crate::memory::r#virtual::Mapping {
            virtual_page,
            physical_page,
            flags: PageFlags::WRITABLE.or(PageFlags::PRESENT),
        }));
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn virtual_space_rejects_duplicate_mappings() {
        let mut space = VirtualAddressSpace::<2>::new();
        let v1 = PageAddress::new(0x1000).unwrap();
        let p1 = PageAddress::new(0x2000).unwrap();
        let p2 = PageAddress::new(0x3000).unwrap();

        space.map_page(v1, p1, PageFlags::USER).unwrap();
        assert_eq!(space.map_page(v1, p2, PageFlags::USER), Err(VirtualMemoryError::PageAlreadyMapped));
        assert_eq!(space.translate(v1), Some(p1));
    }

    #[test]
    fn mapping_table_unmap_removes_existing_mapping() {
        let mut table = MappingTable::<2>::new();
        let virtual_page = PageAddress::new(0x4000).unwrap();
        let physical_page = PageAddress::new(0x5000).unwrap();

        table.map(virtual_page, physical_page, PageFlags::empty()).unwrap();
        let removed = table.unmap(virtual_page).unwrap();
        assert_eq!(removed.virtual_page, virtual_page);
        assert_eq!(removed.physical_page, physical_page);
        assert_eq!(table.get(virtual_page), None);
    }
}

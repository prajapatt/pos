#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentSelector {
    pub raw: u16,
}

impl SegmentSelector {
    pub const fn new(index: u16, rpl: u16) -> Self {
        Self { raw: (index << 3) | (rpl & 0b11) }
    }

    pub const fn index(&self) -> u16 {
        self.raw >> 3
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GdtEntry {
    pub value: u64,
}

impl GdtEntry {
    pub const fn null() -> Self {
        Self { value: 0 }
    }

    pub const fn code_segment() -> Self {
        Self {
            value: 0x00AF9A000000FFFF,
        }
    }

    pub const fn data_segment() -> Self {
        Self {
            value: 0x00AF92000000FFFF,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlobalDescriptorTable {
    entries: [u64; 8],
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        Self { entries: [0; 8] }
    }

    pub fn set_entry(&mut self, index: usize, entry: u64) {
        if index < self.entries.len() {
            self.entries[index] = entry;
        }
    }

    pub fn base(&self) -> *const u64 {
        self.entries.as_ptr()
    }

    pub fn kernel_segments(&mut self) -> (SegmentSelector, SegmentSelector) {
        self.entries[1] = GdtEntry::code_segment().value;
        self.entries[2] = GdtEntry::data_segment().value;
        (
            SegmentSelector::new(1, 0),
            SegmentSelector::new(2, 0),
        )
    }
}

impl Default for GlobalDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

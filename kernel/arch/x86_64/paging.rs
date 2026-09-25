#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageAddress {
    pub value: u64,
}

impl PageAddress {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn aligned4k(&self) -> bool {
        self.value % 0x1000 == 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTableEntry {
    pub raw: u64,
}

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self { raw: 0 }
    }

    pub fn present(&self) -> bool {
        self.raw & 1 != 0
    }

    pub fn set_present(&mut self, present: bool) {
        if present {
            self.raw |= 1;
        } else {
            self.raw &= !1;
        }
    }

    pub fn set_flags(&mut self, flags: u64) {
        self.raw = (self.raw & !0xFFF) | (flags & 0xFFF);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageMapLevel4 {
    entries: [PageTableEntry; 512],
}

impl PageMapLevel4 {
    pub const fn new() -> Self {
        Self { entries: [PageTableEntry::empty(); 512] }
    }

    pub fn identity_map(&mut self, address: u64, flags: u64) {
        let index = (address >> 39) & 0x1FF;
        self.entries[index as usize].raw = (address & !0xFFF) | (flags | 1);
    }
}

impl Default for PageMapLevel4 {
    fn default() -> Self {
        Self::new()
    }
}

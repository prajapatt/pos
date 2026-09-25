#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateType {
    InterruptGate = 0x8E,
    TrapGate = 0x8F,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn empty() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, selector: u16, handler: u64, gate_type: GateType, ist: u8) {
        self.offset_low = (handler & 0xffff) as u16;
        self.selector = selector;
        self.ist = ist;
        self.type_attr = gate_type as u8;
        self.offset_mid = ((handler >> 16) & 0xffff) as u16;
        self.offset_high = ((handler >> 32) & 0xffff_ffff) as u32;
        self.reserved = 0;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterruptDescriptorTable {
    entries: [IdtEntry; 256],
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            entries: [IdtEntry::empty(); 256],
        }
    }

    pub fn set_gate(&mut self, vector: usize, selector: u16, handler: u64, gate_type: GateType, ist: u8) {
        if vector < self.entries.len() {
            self.entries[vector].set_handler(selector, handler, gate_type, ist);
        }
    }

    pub fn ptr(&self) -> IdtPointer {
        IdtPointer {
            limit: (core::mem::size_of::<Self>() - 1) as u16,
            base: self.entries.as_ptr() as u64,
        }
    }
}

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

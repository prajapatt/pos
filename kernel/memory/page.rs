pub const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageAddress(u64);

impl PageAddress {
	pub const fn new(address: u64) -> Option<Self> { if address & (PAGE_SIZE - 1) == 0 { Some(Self(address)) } else { None } }
	pub const fn value(self) -> u64 { self.0 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageFlags(u64);

impl PageFlags {
	pub const PRESENT: Self = Self(1);
	pub const WRITABLE: Self = Self(1 << 1);
	pub const USER: Self = Self(1 << 2);
	pub const NO_EXECUTE: Self = Self(1 << 63);
	pub const fn empty() -> Self { Self(0) }
	pub const fn bits(self) -> u64 { self.0 }
	pub const fn contains(self, other: Self) -> bool { self.0 & other.0 == other.0 }
	pub const fn or(self, other: Self) -> Self { Self(self.0 | other.0) }
}

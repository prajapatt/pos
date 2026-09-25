use super::page::{PageAddress, PAGE_SIZE};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhysicalError { InvalidRange, Exhausted, InvalidAddress, AlreadyFree }

pub struct PhysicalAllocator<const MAX: usize> { base: u64, count: usize, used: [bool; MAX] }

impl<const MAX: usize> PhysicalAllocator<MAX> {
	pub const fn new() -> Self { Self { base: 0, count: 0, used: [false; MAX] } }
	pub fn initialize(&mut self, base: u64, count: usize) -> Result<(), PhysicalError> {
		PageAddress::new(base).ok_or(PhysicalError::InvalidRange)?;
		if count > MAX { return Err(PhysicalError::InvalidRange); }
		self.base = base;
		self.count = count;
		self.used = [false; MAX];
		Ok(())
	}
	pub fn allocate(&mut self) -> Result<PageAddress, PhysicalError> {
		for index in 0..self.count {
			if !self.used[index] {
				self.used[index] = true;
				return PageAddress::new(self.base + index as u64 * PAGE_SIZE).ok_or(PhysicalError::Exhausted);
			}
		}
		Err(PhysicalError::Exhausted)
	}
	pub fn release(&mut self, page: PageAddress) -> Result<(), PhysicalError> { let address = page.value(); if address < self.base || (address - self.base) % PAGE_SIZE != 0 { return Err(PhysicalError::InvalidAddress); } let index = ((address - self.base) / PAGE_SIZE) as usize; if index >= self.count { return Err(PhysicalError::InvalidAddress); } if !self.used[index] { return Err(PhysicalError::AlreadyFree); } self.used[index] = false; Ok(()) }
	pub fn allocated(&self) -> usize { self.used[..self.count].iter().filter(|used| **used).count() }
}

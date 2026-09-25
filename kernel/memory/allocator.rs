pub const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FrameState {
	Free,
	Used,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameAllocatorError {
	InvalidRange,
	Exhausted,
	InvalidFrame,
	FrameAlreadyFree,
}

pub struct FrameAllocator<const MAX_FRAMES: usize> {
	base_address: u64,
	frame_count: usize,
	frames: [FrameState; MAX_FRAMES],
}

impl<const MAX_FRAMES: usize> FrameAllocator<MAX_FRAMES> {
	pub const fn new() -> Self {
		Self {
			base_address: 0,
			frame_count: 0,
			frames: [FrameState::Free; MAX_FRAMES],
		}
	}

	pub fn initialize(&mut self, base_address: u64, frame_count: usize) -> Result<(), FrameAllocatorError> {
		if base_address % PAGE_SIZE != 0 || frame_count > MAX_FRAMES {
			return Err(FrameAllocatorError::InvalidRange);
		}
		let byte_length = (frame_count as u64)
			.checked_mul(PAGE_SIZE)
			.ok_or(FrameAllocatorError::InvalidRange)?;
		base_address
			.checked_add(byte_length)
			.ok_or(FrameAllocatorError::InvalidRange)?;

		self.base_address = base_address;
		self.frame_count = frame_count;
		self.frames = [FrameState::Free; MAX_FRAMES];
		Ok(())
	}

	pub fn allocate(&mut self) -> Result<u64, FrameAllocatorError> {
		for index in 0..self.frame_count {
			if self.frames[index] == FrameState::Free {
				self.frames[index] = FrameState::Used;
				return Ok(self.base_address + (index as u64) * PAGE_SIZE);
			}
		}
		Err(FrameAllocatorError::Exhausted)
	}

	pub fn release(&mut self, address: u64) -> Result<(), FrameAllocatorError> {
		let index = self.frame_index(address).ok_or(FrameAllocatorError::InvalidFrame)?;
		if self.frames[index] == FrameState::Free {
			return Err(FrameAllocatorError::FrameAlreadyFree);
		}
		self.frames[index] = FrameState::Free;
		Ok(())
	}

	pub fn capacity(&self) -> usize {
		self.frame_count
	}

	pub fn allocated(&self) -> usize {
		self.frames[..self.frame_count]
			.iter()
			.filter(|state| **state == FrameState::Used)
			.count()
	}

	fn frame_index(&self, address: u64) -> Option<usize> {
		if address < self.base_address || (address - self.base_address) % PAGE_SIZE != 0 {
			return None;
		}
		let index = ((address - self.base_address) / PAGE_SIZE) as usize;
		(index < self.frame_count).then_some(index)
	}
}

impl<const MAX_FRAMES: usize> Default for FrameAllocator<MAX_FRAMES> {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::{FrameAllocator, FrameAllocatorError, PAGE_SIZE};

	#[test]
	fn allocates_aligned_frames_and_reuses_released_frames() {
		let mut allocator = FrameAllocator::<3>::new();
		allocator.initialize(0x10000, 3).unwrap();

		assert_eq!(allocator.allocate(), Ok(0x10000));
		assert_eq!(allocator.allocate(), Ok(0x10000 + PAGE_SIZE));
		assert_eq!(allocator.allocated(), 2);
		allocator.release(0x10000).unwrap();
		assert_eq!(allocator.allocate(), Ok(0x10000));
	}

	#[test]
	fn rejects_invalid_ranges_and_frames() {
		let mut allocator = FrameAllocator::<2>::new();
		assert_eq!(allocator.initialize(1, 1), Err(FrameAllocatorError::InvalidRange));
		assert_eq!(allocator.initialize(0x1000, 3), Err(FrameAllocatorError::InvalidRange));
		allocator.initialize(0x1000, 1).unwrap();
		assert_eq!(allocator.release(0x2000), Err(FrameAllocatorError::InvalidFrame));
		assert_eq!(allocator.release(0x1001), Err(FrameAllocatorError::InvalidFrame));
	}

	#[test]
	fn reports_exhaustion_and_double_release() {
		let mut allocator = FrameAllocator::<1>::new();
		allocator.initialize(0, 1).unwrap();
		let address = allocator.allocate().unwrap();
		assert_eq!(allocator.allocate(), Err(FrameAllocatorError::Exhausted));
		allocator.release(address).unwrap();
		assert_eq!(allocator.release(address), Err(FrameAllocatorError::FrameAlreadyFree));
	}
}

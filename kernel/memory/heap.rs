use core::alloc::Layout;

pub struct BumpHeap<const SIZE: usize> { storage: [u8; SIZE], offset: usize }

impl<const SIZE: usize> BumpHeap<SIZE> {
	pub const fn new() -> Self { Self { storage: [0; SIZE], offset: 0 } }
	pub fn allocate(&mut self, layout: Layout) -> Option<*mut u8> { let align = layout.align(); let aligned = (self.offset + align - 1) & !(align - 1); let end = aligned.checked_add(layout.size())?; if end > SIZE { return None; } self.offset = end; Some(self.storage.as_mut_ptr().wrapping_add(aligned)) }
	pub const fn used(&self) -> usize { self.offset }
	pub const fn remaining(&self) -> usize { SIZE - self.offset }
}

impl<const SIZE: usize> Default for BumpHeap<SIZE> { fn default() -> Self { Self::new() } }

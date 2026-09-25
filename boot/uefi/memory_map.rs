use core::slice;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryDescriptor {
	pub base: u64,
	pub length: u64,
	pub kind: u32,
}

pub unsafe fn descriptors<'a>(address: *const MemoryDescriptor, count: usize) -> &'a [MemoryDescriptor] {
	// The caller must provide a firmware-owned buffer valid for the boot service lifetime.
	slice::from_raw_parts(address, count)
}

pub fn usable_bytes(descriptors: &[MemoryDescriptor], usable_kind: u32) -> u64 {
	descriptors.iter().filter(|descriptor| descriptor.kind == usable_kind).map(|descriptor| descriptor.length).sum()
}

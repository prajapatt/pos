#![no_std]

pub const BOOT_INFO_VERSION: u32 = 1;
pub const MEMORY_USABLE: u32 = 7;
pub const BOOT_MAGIC: u64 = 0x4348_5554_4f53_424f;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryRegion {
	pub base: u64,
	pub length: u64,
	pub kind: u32,
	pub reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FramebufferInfo {
	pub address: u64,
	pub width: u32,
	pub height: u32,
	pub stride: u32,
	pub format: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootInfo {
	pub version: u32,
	pub size: u32,
	pub kernel_base: u64,
	pub kernel_size: u64,
	pub memory_regions: *const MemoryRegion,
	pub memory_region_count: u32,
	pub reserved: u32,
	pub framebuffer: FramebufferInfo,
}

impl BootInfo {
	pub const fn empty() -> Self {
		Self {
			version: BOOT_INFO_VERSION,
			size: core::mem::size_of::<Self>() as u32,
			kernel_base: 0,
			kernel_size: 0,
			memory_regions: core::ptr::null(),
			memory_region_count: 0,
			reserved: 0,
			framebuffer: FramebufferInfo {
				address: 0,
				width: 0,
				height: 0,
				stride: 0,
				format: 0,
			},
		}
	}

	pub const fn is_valid_header(&self) -> bool {
		self.version == BOOT_INFO_VERSION && self.size >= core::mem::size_of::<Self>() as u32
	}
}

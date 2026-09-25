#![no_std]
#![no_main]

use uefi::prelude::*;

mod framebuffer;
mod loader;
mod memory_map;
#[path = "../../common/boot_info.rs"]
mod boot_info;

#[entry]
fn main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
	let _ = system_table.stdout().write_str("POS UEFI loader\r\n");
	let _ = system_table.stdout().write_str("kernel image loading is not yet connected\r\n");
	Status::UNSUPPORTED
}

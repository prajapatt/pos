use core::arch::asm;

pub struct SerialPort {
	base: u16,
}

impl SerialPort {
	pub const fn com1() -> Self {
		Self { base: 0x3f8 }
	}

	pub unsafe fn initialize(&self) {
		outb(self.base + 1, 0);
		outb(self.base + 3, 0x80);
		outb(self.base, 1);
		outb(self.base + 1, 0);
		outb(self.base + 3, 3);
		outb(self.base + 2, 0xc7);
		outb(self.base + 4, 0x0b);
	}

	pub unsafe fn write_byte(&self, byte: u8) {
		while inb(self.base + 5) & 0x20 == 0 {
			core::hint::spin_loop();
		}
		outb(self.base, byte);
	}

	pub unsafe fn write_str(&self, text: &str) {
		for byte in text.bytes() {
			self.write_byte(byte);
		}
	}
}

unsafe fn outb(port: u16, value: u8) {
	asm!(
		"out dx, al",
		in("dx") port,
		in("al") value,
		options(nostack, preserves_flags)
	);
}

unsafe fn inb(port: u16) -> u8 {
	let value: u8;
	asm!(
		"in al, dx",
		in("dx") port,
		out("al") value,
		options(nostack, preserves_flags)
	);
	value
}
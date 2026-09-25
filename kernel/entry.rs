use crate::arch::x86_64::serial::SerialPort;
use crate::core::kernel::Kernel;
use crate::core::panic::report_panic;

#[no_mangle]
pub extern "C" fn kernel_entry() -> ! {
	let serial = SerialPort::com1();
	unsafe {
		serial.initialize();
		serial.write_str("POS: Rust kernel entry\r\n");
	}

	let mut kernel = Kernel::<64>::new();
	kernel.boot();
	if kernel.spawn(1).is_err() {
		unsafe {
			serial.write_str("POS: scheduler admission failed\r\n");
		}
		loop {
			core::hint::spin_loop();
		}
	}
	unsafe {
		serial.write_str("POS: scheduler online\r\n");
	}

	loop {
		let _ = kernel.tick();
		core::hint::spin_loop();
	}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
	let message = info
		.payload()
		.downcast_ref::<&str>()
		.map(|msg| *msg)
		.unwrap_or("kernel panic");
	report_panic(message);
	loop {
		core::hint::spin_loop();
	}
}
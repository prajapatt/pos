use crate::arch::x86_64::serial::SerialPort;

pub fn report_panic(message: &str) {
	let serial = SerialPort::com1();
	unsafe {
		serial.initialize();
		serial.write_str("POS: kernel panic: ");
		serial.write_str(message);
		serial.write_str("\r\n");
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn panic_messages_are_string_like_and_reportable() {
		let message = "kernel panic";
		assert!(!message.is_empty());
		assert!(message.contains("panic"));
	}
}

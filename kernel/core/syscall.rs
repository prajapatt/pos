#[repr(u64)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Syscall { Exit = 1, Yield = 2, Read = 3, Write = 4, Mmap = 5 }
impl TryFrom<u64> for Syscall {
	type Error = ();
	fn try_from(value: u64) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::Exit),
			2 => Ok(Self::Yield),
			3 => Ok(Self::Read),
			4 => Ok(Self::Write),
			5 => Ok(Self::Mmap),
			_ => Err(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Syscall;

	#[test]
	fn syscall_numbers_map_to_valid_kernel_calls() {
		assert_eq!(Syscall::try_from(1), Ok(Syscall::Exit));
		assert_eq!(Syscall::try_from(2), Ok(Syscall::Yield));
		assert_eq!(Syscall::try_from(3), Ok(Syscall::Read));
		assert_eq!(Syscall::try_from(4), Ok(Syscall::Write));
		assert_eq!(Syscall::try_from(5), Ok(Syscall::Mmap));
	}

	#[test]
	fn unknown_syscall_numbers_are_rejected() {
		assert_eq!(Syscall::try_from(0), Err(()));
		assert_eq!(Syscall::try_from(99), Err(()));
	}
}

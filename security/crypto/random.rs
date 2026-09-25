#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RandomSeed {
	pub state: u64,
}

impl RandomSeed {
	pub const fn new(seed: u64) -> Self {
		Self { state: seed.max(1) }
	}

	pub fn next_u64(&mut self) -> u64 {
		self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
		self.state
	}

	pub fn next_u32(&mut self) -> u32 {
		(self.next_u64() >> 32) as u32
	}
}

#[derive(Clone, Debug)]
pub struct SecureRandom {
	generator: RandomSeed,
}

impl SecureRandom {
	pub const fn new(seed: u64) -> Self {
		Self {
			generator: RandomSeed::new(seed),
		}
	}

	pub fn fill(&mut self, out: &mut [u8]) {
		for chunk in out.iter_mut() {
			let word = self.generator.next_u64();
			*chunk = (word & 0xff) as u8;
		}
	}

	pub fn u64(&mut self) -> u64 {
		self.generator.next_u64()
	}

	pub fn bool(&mut self) -> bool {
		(self.u64() & 1) == 1
	}
}

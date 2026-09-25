#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioFormat { pub sample_rate: u32, pub channels: u8, pub bits_per_sample: u8 }

impl AudioFormat { pub const fn bytes_per_second(self) -> Option<u32> { if self.sample_rate == 0 || self.channels == 0 || self.bits_per_sample == 0 || self.bits_per_sample % 8 != 0 { None } else { Some(self.sample_rate * self.channels as u32 * (self.bits_per_sample / 8) as u32) } } }

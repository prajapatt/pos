pub const MAX_FRAME_SAMPLES: usize = 4096;

#[derive(Clone, Debug, PartialEq)]
pub struct AudioFrame {
	pub sample_rate: u32,
	pub channels: u16,
	pub samples: Vec<i16>,
}

impl AudioFrame {
	pub fn new(sample_rate: u32, channels: u16, samples: Vec<i16>) -> Result<Self, &'static str> {
		if sample_rate == 0 || channels == 0 || samples.is_empty() || samples.len() > MAX_FRAME_SAMPLES {
			return Err("invalid audio frame");
		}
		Ok(Self { sample_rate, channels, samples })
	}

	pub fn rms(&self) -> f32 {
		let sum: f64 = self.samples.iter().map(|sample| f64::from(*sample) * f64::from(*sample)).sum();
		(sum / self.samples.len() as f64).sqrt() as f32
	}
}

use super::screenshot::Screenshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaptureError {
	Unavailable(String),
	InvalidFrame(&'static str),
}

pub trait ScreenCaptureBackend {
	fn capture(&mut self) -> Result<Screenshot, CaptureError>;
}

pub struct UnavailableCapture {
	reason: String,
}

impl UnavailableCapture {
	pub fn new(reason: impl Into<String>) -> Self { Self { reason: reason.into() } }
}

impl ScreenCaptureBackend for UnavailableCapture {
	fn capture(&mut self) -> Result<Screenshot, CaptureError> {
		Err(CaptureError::Unavailable(self.reason.clone()))
	}
}

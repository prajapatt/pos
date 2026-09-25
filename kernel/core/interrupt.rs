use crate::interrupt::exception::Exception;
use crate::interrupt::interrupt::{classify, InterruptResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptAction {
	Handle,
	Ignore,
	Panic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterruptFrame {
	pub vector: u8,
	pub error_code: Option<u64>,
	pub instruction_pointer: u64,
	pub stack_pointer: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterruptController {
	last_vector: Option<u8>,
	last_action: InterruptAction,
}

impl InterruptController {
	pub const fn new() -> Self {
		Self {
			last_vector: None,
			last_action: InterruptAction::Ignore,
		}
	}

	pub fn dispatch(&mut self, vector: u8, error_code: Option<u64>, instruction_pointer: u64, stack_pointer: u64) -> InterruptAction {
		self.last_vector = Some(vector);
		match classify(vector) {
			InterruptResult::Fatal => {
				self.last_action = InterruptAction::Panic;
				match Exception::from(vector) {
					Exception::PageFault => {
						self.last_action = InterruptAction::Panic;
					}
					_ => {}
				}
			}
			InterruptResult::Handled => {
				self.last_action = InterruptAction::Handle;
			}
			InterruptResult::Unhandled => {
				self.last_action = InterruptAction::Ignore;
			}
		}

		let _frame = InterruptFrame {
			vector,
			error_code,
			instruction_pointer,
			stack_pointer,
		};
		self.last_action
	}

	pub fn last_vector(&self) -> Option<u8> {
		self.last_vector
	}

	pub fn last_action(&self) -> InterruptAction {
		self.last_action
	}
}

#[cfg(test)]
mod tests {
	use super::{InterruptAction, InterruptController};

	#[test]
	fn faults_are_marked_panic_and_irqs_are_handled() {
		let mut controller = InterruptController::new();
		assert_eq!(controller.dispatch(0, None, 0x1000, 0x2000), InterruptAction::Panic);
		assert_eq!(controller.dispatch(32, None, 0x3000, 0x4000), InterruptAction::Handle);
		assert_eq!(controller.last_vector(), Some(32));
	}
}

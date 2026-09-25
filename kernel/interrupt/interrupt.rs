#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptResult { Handled, Unhandled, Fatal }

pub fn classify(vector: u8) -> InterruptResult { match vector { 0..=31 => InterruptResult::Fatal, 32..=47 => InterruptResult::Handled, _ => InterruptResult::Unhandled } }

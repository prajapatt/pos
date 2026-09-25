#![allow(dead_code)]

use crate::security::audit::events::{AuditEvent, AuditLevel, AuditOutcome, EventLog};

#[derive(Clone, Debug)]
pub struct AuditLogger<const CAPACITY: usize> {
	log: EventLog<CAPACITY>,
	severe_count: usize,
}

impl<const CAPACITY: usize> AuditLogger<CAPACITY> {
	pub const fn new() -> Self {
		Self {
			log: EventLog::new(),
			severe_count: 0,
		}
	}

	pub fn log(&mut self, source: u32, subject: u32, action: u16, level: AuditLevel, outcome: AuditOutcome) {
		let event = AuditEvent::new(self.timestamp(), source, subject, level, outcome, action);
		if matches!(level, AuditLevel::Critical | AuditLevel::Error) {
			self.severe_count += 1;
		}
		self.log.record(event);
	}

	pub fn flush(&mut self) -> usize {
		let count = self.log.len();
		self.log = EventLog::new();
		self.severe_count = 0;
		count
	}

	pub fn severe_events(&self) -> usize {
		self.severe_count
	}

	pub fn entries(&self) -> usize {
		self.log.len()
	}

	fn timestamp(&self) -> u64 {
		0u64
	}
}

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditLevel {
	Info,
	Warning,
	Error,
	Critical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditOutcome {
	Allowed,
	Denied,
	Failed,
	Escalated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuditEvent {
	pub timestamp: u64,
	pub source: u32,
	pub subject: u32,
	pub level: AuditLevel,
	pub outcome: AuditOutcome,
	pub action: u16,
}

impl AuditEvent {
	pub const fn new(
		timestamp: u64,
		source: u32,
		subject: u32,
		level: AuditLevel,
		outcome: AuditOutcome,
		action: u16,
	) -> Self {
		Self {
			timestamp,
			source,
			subject,
			level,
			outcome,
			action,
		}
	}
}

#[derive(Clone, Debug)]
pub struct EventLog<const CAPACITY: usize> {
	events: [Option<AuditEvent>; CAPACITY],
	write_index: usize,
	count: usize,
}

impl<const CAPACITY: usize> EventLog<CAPACITY> {
	pub const fn new() -> Self {
		const EMPTY: Option<AuditEvent> = None;
		Self {
			events: [EMPTY; CAPACITY],
			write_index: 0,
			count: 0,
		}
	}

	pub fn record(&mut self, event: AuditEvent) {
		self.events[self.write_index] = Some(event);
		self.write_index = (self.write_index + 1) % CAPACITY;
		if self.count < CAPACITY {
			self.count += 1;
		}
	}

	pub fn len(&self) -> usize {
		self.count
	}

	pub fn is_empty(&self) -> bool {
		self.count == 0
	}

	pub fn events(&self) -> impl Iterator<Item = &AuditEvent> {
		let mut seen = 0usize;
		let mut index = self.write_index;

		core::iter::from_fn(move || {
			if seen >= self.count {
				return None;
			}

			let event = &self.events[index];
			index = (index + 1) % CAPACITY;
			seen += 1;

			event.as_ref()
		})
	}
}

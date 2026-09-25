const MAX_MESSAGES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
	pub role: String,
	pub content: String,
}

#[derive(Clone, Debug, Default)]
pub struct ConversationMemory {
	messages: Vec<Message>,
}

impl ConversationMemory {
	pub const fn new() -> Self {
		Self { messages: Vec::new() }
	}

	pub fn push(&mut self, role: impl Into<String>, content: impl Into<String>) {
		if self.messages.len() == MAX_MESSAGES {
			self.messages.remove(0);
		}
		self.messages.push(Message {
			role: role.into(),
			content: content.into(),
		});
	}

	pub fn messages(&self) -> &[Message] {
		&self.messages
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChatRole {
	User,
	Assistant,
	System,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChatMessage {
	pub role: ChatRole,
	pub content: String,
}

#[derive(Default)]
pub struct ChatSession {
	messages: Vec<ChatMessage>,
}

impl ChatSession {
	pub fn push(&mut self, role: ChatRole, content: impl Into<String>) {
		self.messages.push(ChatMessage { role, content: content.into() });
	}

	pub fn messages(&self) -> &[ChatMessage] {
		&self.messages
	}
}

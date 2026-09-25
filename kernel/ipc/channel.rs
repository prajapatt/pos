pub const CHANNEL_CAPACITY: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelError { Full, Empty, PayloadTooLarge }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Message { pub sender: u64, pub length: usize, pub payload: [u8; 64] }

pub struct Channel { queue: [Option<Message>; CHANNEL_CAPACITY], head: usize, tail: usize, count: usize }
impl Channel { pub const fn new() -> Self { Self { queue: [None; CHANNEL_CAPACITY], head: 0, tail: 0, count: 0 } } pub fn send(&mut self, sender: u64, payload: &[u8]) -> Result<(), ChannelError> { if payload.len() > 64 { return Err(ChannelError::PayloadTooLarge); } if self.count == CHANNEL_CAPACITY { return Err(ChannelError::Full); } let mut bytes = [0; 64]; bytes[..payload.len()].copy_from_slice(payload); self.queue[self.tail] = Some(Message { sender, length: payload.len(), payload: bytes }); self.tail = (self.tail + 1) % CHANNEL_CAPACITY; self.count += 1; Ok(()) } pub fn receive(&mut self) -> Result<Message, ChannelError> { let message = self.queue[self.head].take().ok_or(ChannelError::Empty)?; self.head = (self.head + 1) % CHANNEL_CAPACITY; self.count -= 1; Ok(message) } }

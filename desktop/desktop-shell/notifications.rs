#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notification { pub id: u64, pub title: String, pub body: String, pub read: bool }
#[derive(Default)]
pub struct NotificationCenter { items: Vec<Notification> }
impl NotificationCenter { pub fn push(&mut self, notification: Notification) { self.items.push(notification); } pub fn mark_read(&mut self, id: u64) { if let Some(item) = self.items.iter_mut().find(|item| item.id == id) { item.read = true; } } pub fn unread(&self) -> usize { self.items.iter().filter(|item| !item.read).count() } }

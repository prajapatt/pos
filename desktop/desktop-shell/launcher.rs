#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppEntry { pub id: String, pub name: String, pub command: String }
#[derive(Default)]
pub struct Launcher { entries: Vec<AppEntry> }
impl Launcher { pub fn register(&mut self, entry: AppEntry) { self.entries.push(entry); } pub fn find(&self, id: &str) -> Option<&AppEntry> { self.entries.iter().find(|entry| entry.id == id) } pub fn entries(&self) -> &[AppEntry] { &self.entries } }

#[derive(Default)]
pub struct Dock { pinned: Vec<String> }
impl Dock { pub fn pin(&mut self, app_id: impl Into<String>) { let id = app_id.into(); if !self.pinned.contains(&id) { self.pinned.push(id); } } pub fn pinned(&self) -> &[String] { &self.pinned } }

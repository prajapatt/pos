#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivityState {
	Queued,
	Running,
	WaitingForApproval,
	Succeeded,
	Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Activity {
	pub id: u64,
	pub operation: String,
	pub state: ActivityState,
	pub detail: String,
}

#[derive(Default)]
pub struct ActivityLog {
	entries: Vec<Activity>,
}

impl ActivityLog {
	pub fn push(&mut self, activity: Activity) {
		self.entries.push(activity);
	}

	pub fn entries(&self) -> &[Activity] {
		&self.entries
	}
}

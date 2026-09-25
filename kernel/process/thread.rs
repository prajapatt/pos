#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadState { New, Runnable, Running, Blocked, Exited }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Thread { pub id: u64, pub process_id: u64, pub state: ThreadState, pub instruction_pointer: u64, pub stack_pointer: u64 }
impl Thread { pub const fn new(id: u64, process_id: u64, entry: u64, stack: u64) -> Self { Self { id, process_id, state: ThreadState::New, instruction_pointer: entry, stack_pointer: stack } } pub fn make_runnable(&mut self) { self.state = ThreadState::Runnable; } }

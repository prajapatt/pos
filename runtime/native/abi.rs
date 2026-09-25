#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeCallFrame {
    pub arg_count: u8,
    pub stack_pointer: u64,
    pub return_address: u64,
}

impl NativeCallFrame {
    pub const fn new(arg_count: u8, stack_pointer: u64, return_address: u64) -> Self {
        Self { arg_count, stack_pointer, return_address }
    }

    pub fn valid(&self) -> bool {
        self.stack_pointer != 0 && self.return_address != 0
    }
}

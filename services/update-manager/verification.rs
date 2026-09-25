#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationStatus {
    Passed,
    Failed,
    Pending,
}

#[derive(Clone, Debug, Default)]
pub struct VerificationPolicy {
    status: VerificationStatus,
}

impl VerificationPolicy {
    pub fn check(&mut self) -> VerificationStatus {
        self.status = VerificationStatus::Passed;
        self.status
    }
}

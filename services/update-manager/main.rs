mod rollback;
mod updater;
mod verification;

use rollback::{RollbackPlan, RollbackState};
use updater::{UpdateQueue, UpdateStage, UpdateTarget};
use verification::{VerificationPolicy, VerificationStatus};

#[derive(Clone, Debug, Default)]
pub struct UpdateManager {
    queue: UpdateQueue,
    policy: VerificationPolicy,
    rollback: RollbackPlan,
}

impl UpdateManager {
    pub fn enqueue(&mut self, target: UpdateTarget) {
        self.queue.enqueue(target);
    }

    pub fn verify(&mut self) -> VerificationStatus {
        self.policy.check()
    }

    pub fn rollback(&mut self) -> RollbackState {
        self.rollback.prepare()
    }
}

fn main() {
    let mut manager = UpdateManager::default();
    manager.enqueue(UpdateTarget::new("kernel", "1.2.3"));
    println!("queue size: {}", manager.queue.len());
}

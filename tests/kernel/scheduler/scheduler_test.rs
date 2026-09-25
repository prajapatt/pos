#[path = "../../../kernel/core/scheduler.rs"]
mod scheduler;

#[test]
fn scheduler_round_robin_runs_ready_tasks_in_order() {
    let mut scheduler = scheduler::Scheduler::<4>::new();
    scheduler.spawn(10).unwrap();
    scheduler.spawn(20).unwrap();

    assert_eq!(scheduler.schedule_next(), Some(10));
    assert_eq!(scheduler.schedule_next(), Some(20));
    assert_eq!(scheduler.schedule_next(), Some(10));
}

#[test]
fn scheduler_blocks_and_reaps_exited_tasks() {
    let mut scheduler = scheduler::Scheduler::<2>::new();
    scheduler.spawn(1).unwrap();
    scheduler.spawn(2).unwrap();

    scheduler.block(2).unwrap();
    assert_eq!(scheduler.state(2).unwrap(), scheduler::TaskState::Blocked);

    scheduler.exit(1).unwrap();
    scheduler.reap(1).unwrap();
    assert!(scheduler.state(1).is_err());
}

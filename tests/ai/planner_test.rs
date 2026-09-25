extern crate pos_kernel;

use pos_kernel::ai::agent::planner::Planner;
use pos_kernel::ai::agent::reasoning::{classify, Intent};

#[test]
fn classify_reads_system_requests_as_intent() {
    assert_eq!(classify("read file /etc/hosts"), Intent::ReadFile("/etc/hosts".to_string()));
    assert_eq!(classify("create folder /tmp/demo"), Intent::CreateFolder("/tmp/demo".to_string()));
    assert_eq!(classify("list processes"), Intent::ListProcesses);
}

#[test]
fn planner_creates_valid_action_for_system_info() {
    let plan = Planner::create(Intent::ShowSystemInfo).unwrap();
    assert_eq!(plan.steps.len(), 1);
    assert_eq!(plan.steps[0].request.operation, "system.info");
}

#[test]
fn planner_rejects_unknown_intent() {
    assert!(Planner::create(Intent::Unknown).is_none());
}

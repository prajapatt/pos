#[path = "../../../kernel/core/syscall.rs"]
mod syscall;

#[test]
fn syscall_numbers_match_kernel_dispatch_table() {
    assert_eq!(u64::from(syscall::Syscall::Exit as u64), 1);
    assert_eq!(u64::from(syscall::Syscall::Yield as u64), 2);
    assert_eq!(u64::from(syscall::Syscall::Read as u64), 3);
    assert_eq!(u64::from(syscall::Syscall::Write as u64), 4);
    assert_eq!(u64::from(syscall::Syscall::Mmap as u64), 5);
}

#[test]
fn syscall_try_from_rejects_unknown_values() {
    assert!(syscall::Syscall::try_from(999).is_err());
    assert!(syscall::Syscall::try_from(0).is_err());
    assert!(syscall::Syscall::try_from(5).is_ok());
}

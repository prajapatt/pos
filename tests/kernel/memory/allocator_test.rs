#[path = "../../../kernel/memory/allocator.rs"]
mod allocator;

#[test]
fn allocator_tracks_frame_usage_and_recovery() {
    let mut allocator = allocator::FrameAllocator::<4>::new();
    allocator.initialize(0x2000, 4).unwrap();

    let frame1 = allocator.allocate().unwrap();
    let frame2 = allocator.allocate().unwrap();
    assert_eq!(frame1, 0x2000);
    assert_eq!(frame2, 0x3000);
    assert_eq!(allocator.allocated(), 2);

    allocator.release(frame1).unwrap();
    assert_eq!(allocator.allocated(), 1);
    assert_eq!(allocator.allocate().unwrap(), 0x2000);
}

#[test]
fn allocator_rejects_invalid_ranges_and_double_release() {
    let mut allocator = allocator::FrameAllocator::<2>::new();
    assert!(allocator.initialize(1, 1).is_err());
    assert!(allocator.initialize(0x1000, 3).is_err());

    allocator.initialize(0x4000, 2).unwrap();
    let addr = allocator.allocate().unwrap();
    assert!(allocator.release(addr).is_ok());
    assert!(allocator.release(addr).is_err());
}

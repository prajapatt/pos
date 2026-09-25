#[path = "../../../kernel/ipc/channel.rs"]
mod channel;

#[test]
fn channel_transfers_message_payloads_in_fifo_order() {
    let mut channel = channel::Channel::new();

    channel.send(42, b"hello").unwrap();
    channel.send(99, b"world").unwrap();

    let first = channel.receive().unwrap();
    assert_eq!(first.sender, 42);
    assert_eq!(&first.payload[..5], b"hello");

    let second = channel.receive().unwrap();
    assert_eq!(second.sender, 99);
    assert_eq!(&second.payload[..5], b"world");
}

#[test]
fn channel_rejects_overflow_and_empty_reads() {
    let mut channel = channel::Channel::new();
    for idx in 0..channel::CHANNEL_CAPACITY {
        channel.send(idx as u64, &[idx as u8; 1]).unwrap();
    }

    assert!(channel.send(123, b"x").is_err());
    assert!(channel.receive().is_ok());
    assert!(channel.receive().is_ok());
}

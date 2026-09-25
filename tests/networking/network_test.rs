#[path = "../../networking/tcp/socket.rs"]
mod tcp_socket;

#[test]
fn tcp_socket_starts_closed_and_connects() {
    let mut socket = tcp_socket::TcpSocket::new(8080);
    assert_eq!(socket.state, tcp_socket::TcpState::Closed);

    socket.connect([10, 0, 0, 1], 80);
    assert_eq!(socket.state, tcp_socket::TcpState::SynSent);
    assert_eq!(socket.remote_ip, [10, 0, 0, 1]);
    assert_eq!(socket.remote_port, 80);
}

#[test]
fn tcp_socket_can_accept_and_close() {
    let mut socket = tcp_socket::TcpSocket::new(9000);
    socket.accept();
    assert_eq!(socket.state, tcp_socket::TcpState::Listen);

    socket.close();
    assert_eq!(socket.state, tcp_socket::TcpState::Closed);
}

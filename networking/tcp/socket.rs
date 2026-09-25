#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TcpSocket {
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_ip: [u8; 4],
    pub state: TcpState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    Established,
    FinWait,
}

impl TcpSocket {
    pub const fn new(local_port: u16) -> Self {
        Self {
            local_port,
            remote_port: 0,
            remote_ip: [0; 4],
            state: TcpState::Closed,
        }
    }

    pub fn connect(&mut self, remote_ip: [u8; 4], remote_port: u16) {
        self.remote_ip = remote_ip;
        self.remote_port = remote_port;
        self.state = TcpState::SynSent;
    }

    pub fn accept(&mut self) {
        self.state = TcpState::Listen;
    }

    pub fn close(&mut self) {
        self.state = TcpState::Closed;
    }
}

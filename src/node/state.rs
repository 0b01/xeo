use crypto::Crypto;
use std::net::SocketAddr;

pub struct NodeState {
    pub server_addr: SocketAddr,
    pub rsa: Crypto,
}

impl NodeState {
    pub fn new(server_addr: SocketAddr) -> Self {
        let rsa = Crypto::new();
        Self { server_addr, rsa }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.server_addr.clone()
    }
}

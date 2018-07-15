use std::net::{SocketAddr};

pub struct NodeState {
    server_addr: SocketAddr,
}

impl NodeState {
    pub fn new(server_addr: SocketAddr) -> Self {
        Self {
            server_addr,
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.server_addr.clone()
    }
}

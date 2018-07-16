use crypto::Crypto;
use std::net::SocketAddr;
use std::collections::HashMap;
use node::peer::PeerState;

pub struct NodeState {
    pub server_addr: SocketAddr,
    pub peers: Peers,
    pub rsa: Crypto,
}

impl NodeState {
    pub fn new(server_addr: SocketAddr, peers: Peers) -> Self {
        let rsa = Crypto::new();
        Self { server_addr, rsa, peers }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.server_addr.clone()
    }

    pub fn print_peers_info(&self) {
        println!("{:#?}", self.peers);
    }
}

#[derive(Debug)]
pub struct Peers {
    peers: HashMap<SocketAddr, PeerState>,
}

impl Peers {
    pub fn new(hm: HashMap<SocketAddr, PeerState>) -> Self {
        Peers {
            peers: hm,
        }
    }
}

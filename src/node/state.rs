use crypto::Crypto;
use node::peer::PeerState;
use std::collections::hash_map::{HashMap, Iter};
use std::net::SocketAddr;

pub struct NodeState {
    pub server_addr: SocketAddr,
    pub peers: Peers,
    pub rsa: Crypto,
}

impl NodeState {
    pub fn new(server_addr: SocketAddr, peers: Peers) -> Self {
        let rsa = Crypto::new();
        Self {
            server_addr,
            rsa,
            peers,
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.server_addr.clone()
    }

    pub fn print_peers_info(&self) {
        println!("{:?}", self.peers);
    }
}

#[derive(Debug)]
pub struct Peers {
    peers: HashMap<SocketAddr, PeerState>,
}

impl Peers {
    pub fn new(hm: HashMap<SocketAddr, PeerState>) -> Self {
        Peers { peers: hm }
    }

    pub fn set_key(&mut self, addr: &SocketAddr, key: &[u8]) {
        let peer = self
            .peers
            .entry(addr.to_owned())
            .or_insert(PeerState::new());
        peer.set_key(key.to_owned());
    }

    pub fn pairs(&self) -> Iter<SocketAddr, PeerState> {
        self.peers.iter()
    }
}

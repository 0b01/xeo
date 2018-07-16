#[derive(Debug)]
pub struct PeerState {
    pub pubkey: Option<Vec<u8>>,
}

impl PeerState {
    pub fn new() -> Self {
        PeerState {
            pubkey: None,
        }
    }
}

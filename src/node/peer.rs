#[derive(Debug)]
pub struct PeerState {
    pub pubkey: Option<Vec<u8>>,
}

impl PeerState {
    pub fn new() -> Self {
        PeerState { pubkey: None }
    }

    // pub fn with_pubkey(key: Vec<u8>) -> Self {
    //     PeerState { pubkey: Some(key) }
    // }

    pub fn set_key(&mut self, key: Vec<u8>) {
        self.pubkey = Some(key.to_owned());
    }
}

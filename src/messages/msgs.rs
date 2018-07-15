use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkRequest {
    GetPubKey {
        src: SocketAddr,
        req: PubKeyRequest
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkResponse {
    PubKeyResponse(SocketAddr, PubKeyResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubKeyResponse {
    pubkey: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubKeyRequest {
}

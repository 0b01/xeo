use std::net::SocketAddr;

type SrcAddr = SocketAddr;
type DstAddr = SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkRequest {
    GetPubKey {
        src: SrcAddr,
        req: PubKeyRequest
    },
    PubKeyResponse{
        src: SrcAddr,
        key: PubKeyResponse
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubKeyResponse {
    pubkey: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubKeyRequest {
}

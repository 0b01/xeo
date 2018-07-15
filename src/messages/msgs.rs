use std::net::SocketAddr;

type SrcAddr = SocketAddr;
type DstAddr = SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkRequest {
    GetPubKey { src: SrcAddr },
    PubKeyResponse { src: SrcAddr, key: Vec<u8> },
}

impl NetworkRequest {
    pub fn ty(&self) -> &'static str {
        use self::NetworkRequest::*;
        match self {
            GetPubKey { .. } => "GetPubKey",
            PubKeyResponse { .. } => "PubKeyResponse",
        }
    }
}

use messages::msgs::NetworkRequest;
use node::state::NodeState;
use std::net::SocketAddr;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type ST = Arc<Mutex<NodeState>>;
type SrcAddr = SocketAddr;
type DstAddr = SocketAddr;

pub struct MsgHandler {
    state: ST,
}

impl MsgHandler {
    pub fn new(state: ST) -> Self {
        Self { state }
    }

    fn handle(
        &self,
        src_addr: &SrcAddr,
        msg: &NetworkRequest,
    ) -> Option<(DstAddr, NetworkRequest)> {
        match msg {
            NetworkRequest::GetPubKey { src } => {
                debug!("received GetPubKey from {}", src);
                let st = self.state.lock().unwrap();
                let pub_key = st.rsa.get_pubkey();
                Some((
                    src.clone(),
                    NetworkRequest::PubKeyResponse {
                        src: st.get_addr(),
                        key: pub_key,
                    },
                ))
            }
            NetworkRequest::PubKeyResponse { src, key } => {
                debug!("received PubKeyResponse from {}", src);
                if src != src_addr {
                    warn!(
                        "Self-claimed origin mismatches with actual origin: {} - {}",
                        src, src_addr
                    );
                }
                self.state.lock().unwrap().peers.set_key(src_addr, key);
                None
            }
        }
    }

    pub fn run(
        self,
        req_rx: Receiver<(SrcAddr, NetworkRequest)>,
        res_tx: Sender<(DstAddr, NetworkRequest)>,
    ) -> JoinHandle<i32> {
        thread::spawn(move || loop {
            match req_rx.recv() {
                Ok((src_addr, req)) => {
                    match self.handle(&src_addr, &req) {
                        Some(res) => res_tx.send(res).unwrap(),
                        None => (),
                    };
                }
                Err(e) => error!("{:#?}", e),
            }
        })
    }
}

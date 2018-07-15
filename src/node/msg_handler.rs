use std::thread;
use std::net::SocketAddr;
use std::sync::mpsc::{Sender, Receiver};
use messages::msgs::NetworkRequest;
use std::sync::{Arc, Mutex};
use node::state::NodeState;

type ST = Arc<Mutex<NodeState>>;
type DstAddr = SocketAddr;

pub struct MsgHandler {
    state: ST,
}

impl MsgHandler {
    pub fn new(state: ST) -> Self {
        Self { state }
    }

    fn handle(&self, msg: &NetworkRequest) -> (DstAddr, NetworkRequest) {
        match msg {
            NetworkRequest::GetPubKey {src, req} => unimplemented!(),
            _ => unimplemented!(),
        }
    }

    pub fn run(self, req_rx: Receiver<NetworkRequest>, res_tx: Sender<(DstAddr, NetworkRequest)>) {
        thread::spawn(move || {
            loop {
                match req_rx.recv() {
                    Ok(req) => {
                        let res = self.handle(&req);
                        res_tx.send(res).unwrap();
                    }
                    Err(e) => error!("{:#?}", e),
                }
            }
        });
    }

}

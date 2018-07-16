use bincode::{deserialize, serialize};
use errors::XEOError;
use messages::msgs::NetworkRequest;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use node::state::NodeState;
use std::sync::{Arc, Mutex};
type ST = Arc<Mutex<NodeState>>;

pub struct UdpHandler {
    udp_socket: UdpSocket,
    state: ST,
}

impl UdpHandler {
    pub fn new(state: ST) -> Result<Self, XEOError> {
        let addr = state.lock().unwrap().get_addr();
        info!("starting xeo node: {}", addr);
        let socket = UdpSocket::bind(&addr)?;
        Ok(Self {
            udp_socket: socket,
            state,
        })
    }
    pub fn run(
        self,
        msg_rx: Receiver<(SocketAddr, NetworkRequest)>,
        req_tx: Sender<NetworkRequest>,
    ) -> () {
        debug!("bootstrapping udp receiver loop...");
        let sock = self.udp_socket.try_clone().unwrap();
        thread::spawn(move || loop {
            let mut buf = [0; 1024];
            match sock.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    let buf = &mut buf[..amt];
                    let req = deserialize::<NetworkRequest>(buf).unwrap();
                    req_tx.send(req).unwrap();
                }
                Err(e) => error!("{:#?}", e),
            }
        });

        thread::spawn(move || loop {
            match msg_rx.recv() {
                Ok((dst, msg)) => {
                    debug!("sending {} to {}", msg.ty(), dst);
                    let ser_msg = &serialize(&msg).unwrap();
                    self.udp_socket.send_to(ser_msg, dst).unwrap();
                }
                Err(e) => error!("{:#?}", e),
            };
        });
    }
}

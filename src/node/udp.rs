use std::net::UdpSocket;
use errors::XEOError;
use std::net::SocketAddr;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use bincode::{deserialize, serialize};
use messages::msgs::NetworkRequest;

use std::sync::{Arc, Mutex};
use node::state::NodeState;
type ST = Arc<Mutex<NodeState>>;


pub struct UdpHandler {
    udp_socket: UdpSocket,
    state: ST
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
    pub fn run(self, msg_rx: Receiver<(SocketAddr, NetworkRequest)>, req_tx: Sender<NetworkRequest>) {
        debug!("bootstrapping udp receiver loop...");
        let sock = self.udp_socket.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buf = [0; 1024];
                match sock.recv_from(&mut buf) {
                    Ok((amt, _src)) => {
                        let buf = &mut buf[..amt];
                        let req = deserialize::<NetworkRequest>(buf).unwrap();
                        req_tx.send(req).unwrap();
                    },
                    Err(e) => error!("{:#?}", e),
                }
            }
        });

        thread::spawn(move || {
            loop {
                match msg_rx.recv() {
                    Ok((dst, msg)) => {
                        let ser_msg = &serialize(&msg).unwrap();
                        self.udp_socket.send_to(ser_msg, dst);
                    }
                    Err(e) => error!("{:#?}", e),
                };
            }
        });
    }
}

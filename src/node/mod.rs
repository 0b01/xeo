pub mod cli;
pub mod msg_handler;
pub mod repl;
pub mod state;
pub mod udp;

use self::msg_handler::MsgHandler;
use self::repl::ReplHandler;
use self::udp::UdpHandler;

use errors::XEOError;
use messages::msgs::NetworkRequest;
use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

type DstAddr = SocketAddr;

pub struct Node {
    // res_rx: Receiver<(DstAddr, NetworkRequest)>,
    msg_tx: Sender<(DstAddr, NetworkRequest)>,
}

impl Node {
    pub fn new() -> Result<Self, XEOError> {
        let opt = cli::Opt::from_args();
        opt.setup_logger()?;

        let udp_addr = format!("127.0.0.1:{}", opt.port);
        let udp_addr = udp_addr.parse::<SocketAddr>()?;

        let (msg_tx, msg_rx) = mpsc::channel();
        let (req_tx, req_rx) = mpsc::channel();
        // let (res_tx, res_rx) = mpsc::channel();

        let st = state::NodeState::new(udp_addr);
        let state = Arc::new(Mutex::new(st));

        let udp_handler = UdpHandler::new(state.clone())?;
        udp_handler.run(msg_rx, req_tx);
        let msg_handler = MsgHandler::new(state.clone());
        msg_handler.run(req_rx, msg_tx.clone());
        let repl_handler = ReplHandler::new(state.clone());
        repl_handler.run(msg_tx.clone())?;

        // for msg in res_rx {
        //     println!("shit");
        //     msg_tx.send(msg).unwrap();
        // }

        Ok(Self {
            // res_rx,
            msg_tx,
        })
    }

    pub fn run(self) {}
}

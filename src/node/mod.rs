pub mod msg_handler;
pub mod udp;
pub mod cli;
pub mod repl;
pub mod state;

use self::msg_handler::MsgHandler;
use self::repl::ReplHandler;
use self::udp::UdpHandler;

use std::net::SocketAddr;
use structopt::StructOpt;
use errors::XEOError;
use std::sync::mpsc::{self, Sender, Receiver};
use messages::msgs::NetworkRequest;
use std::sync::{Arc, Mutex};

type DstAddr = SocketAddr;

pub struct Node {
    res_rx: Receiver<(DstAddr, NetworkRequest)>,
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
        let (res_tx, res_rx) = mpsc::channel();

        let state = Arc::new(Mutex::new(state::NodeState::new(udp_addr)));

        let udp_handler = UdpHandler::new(state.clone())?;
        udp_handler.run(msg_rx, req_tx);
        let msg_handler = MsgHandler::new(state.clone());
        msg_handler.run(req_rx, res_tx);
        let repl_handler = ReplHandler::new(state.clone());
        repl_handler.run(msg_tx.clone())?;


        Ok(Self {
            res_rx,
            msg_tx,
        })
    }

    pub fn run(self) {
        for msg in self.res_rx {
            self.msg_tx.send(msg).unwrap();
        }
    }
}

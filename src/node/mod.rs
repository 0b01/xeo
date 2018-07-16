use std::net::SocketAddr;
use std::sync::mpsc::{self};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod peer;
pub mod cli;
pub mod msg_handler;
pub mod repl;
pub mod state;
pub mod udp;

use self::peer::PeerState;
use self::state::Peers;

use self::msg_handler::MsgHandler;
use self::repl::ReplHandler;
use self::udp::UdpHandler;
use errors::XEOError;
use structopt::StructOpt;


pub struct Node {
}

impl Node {
    pub fn start() -> Result<Self, XEOError> {
        let opt = cli::Opt::from_args();
        opt.setup_logger()?;

        let peers = match opt.path_to_peers_txt {
            None => Peers::new(HashMap::new()),
            Some(path) => read_peers_txt(path)?,
        };

        let udp_addr = format!("127.0.0.1:{}", opt.port);
        let udp_addr = udp_addr.parse::<SocketAddr>()?;

        let (msg_tx, msg_rx) = mpsc::channel();
        let (req_tx, req_rx) = mpsc::channel();

        let st = state::NodeState::new(udp_addr, peers);
        let state = Arc::new(Mutex::new(st));

        let udp_handler = UdpHandler::new(state.clone())?;
        udp_handler.run(msg_rx, req_tx);
        let msg_handler = MsgHandler::new(state.clone());
        let msg_join = msg_handler.run(req_rx, msg_tx.clone());

        if opt.interactive {
            let repl_handler = ReplHandler::new(state.clone());
            repl_handler.run(msg_tx.clone())?;
        } else {
            msg_join.join().unwrap();
        }

        Ok(Self{})
    }
}

fn read_peers_txt(path: PathBuf) -> Result<Peers, XEOError> {
    let f = File::open(path).unwrap();
    let rdr = BufReader::new(f);
    let mut peers = HashMap::new();
    for line in rdr.lines() {
        peers.insert(line?.parse()?, PeerState::new());
    }

    Ok(Peers::new(peers))
}

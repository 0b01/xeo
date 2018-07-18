use errors::XEOError;
use messages::msgs::NetworkRequest;
use node::state::NodeState;
use std::io::{self, BufRead, Write};
use std::net::{AddrParseError, SocketAddr};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

type DstAddr = SocketAddr;
type ST = Arc<Mutex<NodeState>>;

#[derive(Debug)]
pub enum ReplCmd {
    Quit,
    PubKey { dst: String },
    Error { msg: String },
    Peers,
    Unknown,
}

pub struct ReplParser {}
impl ReplParser {
    pub fn parse(line: &str) -> ReplCmd {
        if line == "quit" || line == "exit" || line == "bye" {
            ReplCmd::Quit
        } else if line.starts_with("pubkey") {
            ReplParser::parse_pubkey(line)
        } else if line == "peers" {
            ReplCmd::Peers
        } else {
            ReplCmd::Unknown
        }
    }

    fn parse_pubkey(line: &str) -> ReplCmd {
        let toks = line.trim().split(" ").collect::<Vec<&str>>();
        let addr = toks.get(1);
        match addr {
            Some(addr) => ReplCmd::PubKey {
                dst: addr.to_string(),
            },
            None => ReplCmd::Error {
                msg: "Syntax: pubkey [HOSTNAME]".to_owned(),
            },
        }
    }
}

pub enum ReplError {
    ReplExit,
    ReplUnknownCommand,
    Ignore,
    ReplCmdError(String),
}

impl From<AddrParseError> for ReplError {
    fn from(a: AddrParseError) -> Self {
        ReplError::ReplCmdError(format!("Cannot parse: {}", a))
    }
}

pub struct ReplHandler {
    st: ST,
}

impl ReplHandler {
    pub fn new(st: ST) -> Self {
        Self { st }
    }

    pub fn run(&self, msg_tx: Sender<(DstAddr, NetworkRequest)>) -> Result<(), XEOError> {
        let stdin = io::stdin();
        print!("---> ");
        io::stdout().flush()?;
        while let Some(Ok(line)) = stdin.lock().lines().next() {
            let cmd = ReplParser::parse(line.as_str());
            match self.run_cmd(cmd) {
                Err(ReplError::Ignore) => (),
                Err(ReplError::ReplExit) => {
                    println!("bye.",);
                    break;
                }
                Err(ReplError::ReplUnknownCommand) => {
                    println!("Unknown command.");
                }
                Err(ReplError::ReplCmdError(msg)) => {
                    println!("{}", msg);
                }
                Ok((dst, msg)) => msg_tx.send((dst, msg)).unwrap(),
            };
            print!("---> ");
            io::stdout().flush()?;
        }
        Ok(())
    }

    fn run_cmd(&self, cmd: ReplCmd) -> Result<(DstAddr, NetworkRequest), ReplError> {
        match cmd {
            ReplCmd::Quit => Err(ReplError::ReplExit),
            ReplCmd::Unknown => Err(ReplError::ReplUnknownCommand),
            ReplCmd::Error { msg } => Err(ReplError::ReplCmdError(msg)),
            ReplCmd::Peers => {
                let st = self.st.lock().unwrap();
                st.print_peers_info();
                Err(ReplError::Ignore)
            }
            ReplCmd::PubKey { dst } => Ok((
                dst.parse()?,
                NetworkRequest::GetPubKey {
                    src: self.st.lock().unwrap().get_addr(),
                },
            )),
        }
    }
}

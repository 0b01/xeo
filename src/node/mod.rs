pub mod handler;
pub mod cli;
pub mod repl;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use structopt::StructOpt;
use errors::MDCError;
use std::sync::mpsc;

use self::repl::{ReplParser, ReplCmd};

use std::net::UdpSocket;
use std::sync::mpsc::{Sender, Receiver};
use messages::msgs::{NetworkRequest, PubKeyRequest};
use std::thread;
use bincode::{deserialize, serialize};

use std::io::{self, BufRead, Write};
pub struct Node {
    udp_addr: SocketAddr,
    udp_socket: UdpSocket,
    parser: ReplParser,
    req_tx: Sender<NetworkRequest>,
}

impl Node {
    pub fn new() -> Result<Self, MDCError> {
        let opt = cli::Opt::from_args();
        opt.setup_logger()?;

        let udp_addr = format!("127.0.0.1:{}", opt.port);
        let udp_socket = Node::start_udp(&udp_addr)?;
        let udp_addr = udp_addr.parse::<SocketAddr>()?;

        let (tx, rx) = mpsc::channel();

        let parser = ReplParser::new();
        let handler = handler::NodeHandler::new();
        handler.run(rx);

        Ok(Self {
            udp_addr,
            udp_socket,
            parser,
            req_tx: tx,
        })
    }

    pub fn start_udp(addr: &str) -> Result<UdpSocket, MDCError> {
        info!("starting mdc node: {}", addr);
        let socket = UdpSocket::bind(&addr)?;
        Ok(socket)
    }

    pub fn start_udp_recv(&self) {
        debug!("bootstrapping udp receiver loop...");
        let sock = self.udp_socket.try_clone().unwrap();
        let tx = self.req_tx.clone();
        thread::spawn(move || {
            loop {
                let mut buf = [0; 1024];
                match sock.recv_from(&mut buf) {
                    Ok((amt, _src)) => {
                        let buf = &mut buf[..amt];
                        let req = deserialize::<NetworkRequest>(buf).unwrap();
                        tx.send(req).unwrap();
                    },
                    Err(e) => error!("{:#?}", e),
                }
            }
        });
    }

    pub fn start_repl(&self) -> Result<(), MDCError> {
        let stdin = io::stdin();
        print!("---> "); io::stdout().flush()?;
        while let Some(Ok(line)) = stdin.lock().lines().next() {
            let cmd = self.parser.parse(line.as_str());
            self.run_cmd(cmd)?;
            print!("---> "); io::stdout().flush()?;
        }
        Ok(())
    }

    pub fn send_udp(&self, dst: SocketAddr, msg: &NetworkRequest) -> Result<(), MDCError> {
        let packet = serialize(msg)?;
        self.udp_socket.send_to(&packet, dst)?;
        Ok(())
    }

    pub fn run_cmd(&self, cmd: ReplCmd) -> Result<(), MDCError> {
        match cmd {
            ReplCmd::Quit => return Err(MDCError::ReplExit),
            ReplCmd::Unknown => println!("Unknown command."),
            ReplCmd::Error{msg} => println!("{}", msg),
            ReplCmd::PubKey{dst} => {
                self.send_udp(dst.parse()?, &NetworkRequest::GetPubKey {
                    src: self.udp_addr,
                    req: PubKeyRequest{},
                })?;
            },
            _ => unimplemented!(),
        };
        Ok(())
    }
}

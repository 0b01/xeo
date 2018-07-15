#![feature(try_trait)]

#[macro_use]
extern crate serde_derive;
extern crate bincode;

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;


extern crate openssl;
extern crate oping;

mod crypto;
mod errors;
mod ping;
mod messages;
mod node;

use errors::XEOError;
use node::Node;


fn main() -> Result<(), XEOError> {
    let node = Node::new()?;
    node.start_udp_recv();
    node.start_repl()?;

    Ok(())
}

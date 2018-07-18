#![feature(try_trait)]

#[macro_use]
extern crate serde_derive;
extern crate bincode;

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate chrono;
extern crate fern;

extern crate openssl;
extern crate oping;

mod crypto;
mod errors;
mod messages;
mod node;
mod ping;

use errors::XEOError;
use node::Node;

fn main() -> Result<(), XEOError> {
    Node::start()?;
    Ok(())
}

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
mod cli;
mod msgs;

use errors::MDCError;


fn main() -> Result<(), MDCError> {
    cli::start()
}

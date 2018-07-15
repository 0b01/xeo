#![feature(try_trait)]

extern crate openssl;
extern crate oping;

mod crypto;
mod errors;
mod ping;

use errors::MDCError;

fn main() -> Result<(), MDCError> {
    Ok(())
}

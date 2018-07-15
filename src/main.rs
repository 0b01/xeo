extern crate oping;

mod errors;
mod ping;

use errors::MDCError;


fn main() -> Result<(), MDCError> {
    let hosts = vec!["facebook.com", "localhost"];
    for (host, lat) in ping::run_ping(&hosts)?.iter() {
        println!("{} :: {}ms", host, lat);
    }
    Ok(())
}

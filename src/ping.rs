use oping::{Ping, PingResult};

pub fn run_ping<'a>(hosts: &[&str]) -> PingResult<Vec<(String, f64)>> {
    let mut ping = Ping::new();
    ping.set_timeout(5.)?;
    for h in hosts.iter() {
        ping.add_host(h)?;
    }
    let responses = ping.send()?;
    Ok(responses.map(|x| (x.hostname, x.latency_ms)).collect())
}

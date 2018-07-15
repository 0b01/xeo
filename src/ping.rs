use errors::MDCError;
use oping::Ping;

pub fn run_ping<'a>(hosts: &[&str]) -> Result<Vec<(String, f64)>, MDCError> {
    let mut ping = Ping::new();
    ping.set_timeout(5.)?;
    for h in hosts.iter() {
        ping.add_host(h)?;
    }
    let responses = ping.send()?;
    Ok(responses.map(|x| (x.hostname, x.latency_ms)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run_ping() {
        let hosts = vec!["facebook.com", "localhost"];
        for (host, lat) in run_ping(&hosts).unwrap().iter() {
            println!("{} :: {}ms", host, lat);
        }
    }
}

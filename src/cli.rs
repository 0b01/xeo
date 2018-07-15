use fern::{Dispatch, InitError, log_file};
use log::LevelFilter;
use std::io;
use chrono::Local;
use structopt::StructOpt;

use std::net::UdpSocket;
use errors::MDCError;


#[derive(StructOpt, Debug)]
#[structopt(name = "mdc")]
pub struct Opt {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
    #[structopt(short = "p", long = "port", default_value = "9000")]
    pub port: u16,
}

impl Opt {
    pub fn setup_logger(&self) -> Result<(), InitError> {
        let level = match self.verbose {
            0 => LevelFilter::Debug,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(level)
            .chain(io::stdout())
            .chain(log_file("output.log")?)
            .apply()?;
        Ok(())
    }
}

pub fn start() -> Result<(), MDCError>{
    let opt = Opt::from_args();
    opt.setup_logger()?;

    let port = opt.port;
    info!("starting mdc-cli v0.0.1 on port: {}", port);

    let socket = UdpSocket::bind(format!("localhost:{}", port))?;
    loop {
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
    Ok(())
}

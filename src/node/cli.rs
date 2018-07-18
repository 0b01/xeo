use chrono::Local;
use fern::{log_file, Dispatch, InitError};
use log::LevelFilter;
use std::io;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "xeo")]
pub struct Opt {
    #[structopt(short = "i", long = "interactive")]
    pub interactive: bool,
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
    #[structopt(short = "p", long = "port", default_value = "9000")]
    pub port: u16,
    #[structopt(long = "peerlist", parse(from_os_str))]
    pub path_to_peers_txt: Option<PathBuf>,
}

impl Opt {
    pub fn setup_logger(&self) -> Result<(), InitError> {
        let level = match self.verbose {
            0 => LevelFilter::Debug, // Error
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        let d = Dispatch::new()
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
            .chain(log_file("output.log")?);

        if self.interactive {
            d
        } else {
            d.chain(io::stdout())
        }.apply()?;

        Ok(())
    }
}

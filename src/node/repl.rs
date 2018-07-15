#[derive(Debug)]
pub enum ReplCmd {
    Quit,
    PubKey {
        dst: String,
    },
    Error {
        msg: String
    },
    Unknown,
}

pub struct ReplParser {}
impl ReplParser {
    pub fn new() -> Self {
        Self {}
    }
    pub fn parse(&self, line: &str) -> ReplCmd {
        if line == "quit" || line == "exit" || line == "bye" { ReplCmd::Quit }
        else if line.starts_with("pubkey") { self.parse_pubkey(line) }
        else { ReplCmd::Unknown }
    }

    fn parse_pubkey(&self, line: &str) -> ReplCmd {
        let toks = line.trim().split(" ").collect::<Vec<&str>>();
        let addr = toks.get(1);
        match addr {
            Some(addr) => ReplCmd::PubKey {
                    dst: addr.to_string(),
                },
            None => ReplCmd::Error {
                    msg: "Must supply a dst addr".to_owned()
                }
        }
    }
}

use std::thread;
use std::marker::PhantomData;
// use std::sync::Arc;
use std::sync::mpsc::Receiver;
use messages::msgs::NetworkRequest;

pub struct NodeHandler {
}

impl NodeHandler {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn on(&self, msg: &NetworkRequest) {
        debug!("{:?}", msg);
    }

    pub fn run(self, rx: Receiver<NetworkRequest>) {
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(msg) => self.on(&msg),
                    Err(e) => error!("{:#?}", e),
                }
            }
        });
    }

}

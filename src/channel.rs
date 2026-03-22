use std::collections::VecDeque;

use crate::common::Error;

pub trait Channel {
    fn send(&mut self, json_payload: String) -> Result<(), Error>;
    fn recv() -> Result<Option<String>, Error>;
}

pub struct LocalStorageChannel {
    messages: VecDeque<String>,
}

impl Channel for LocalStorageChannel {
    fn send(&mut self, json_payload: String) -> Result<(), Error> {
        self.messages.push_back(json_payload);
        Ok(())
    }

    fn recv() -> Result<Option<String>, Error> {}
}

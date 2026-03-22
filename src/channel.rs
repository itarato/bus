use std::collections::VecDeque;

use crate::common::Error;

pub trait Channel {
    fn send(&mut self, json_payload: String) -> Result<(), Error>;
    fn recv(&mut self) -> Result<Option<String>, Error>;
}

#[derive(Default)]
pub struct LocalStorageChannel {
    messages: VecDeque<String>,
}

impl Channel for LocalStorageChannel {
    fn send(&mut self, json_payload: String) -> Result<(), Error> {
        self.messages.push_back(json_payload);
        Ok(())
    }

    fn recv(&mut self) -> Result<Option<String>, Error> {
        Ok(self.messages.pop_front())
    }
}

use std::collections::VecDeque;

use crate::{common::Error, message::Message};

pub trait Channel {
    fn send(&mut self, message: Message) -> Result<(), Error>;
    fn recv(&mut self) -> Result<Option<Message>, Error>;
}

#[derive(Default)]
pub struct LocalStorageChannel {
    messages: VecDeque<Message>,
}

impl Channel for LocalStorageChannel {
    fn send(&mut self, message: Message) -> Result<(), Error> {
        self.messages.push_back(message);
        Ok(())
    }

    fn recv(&mut self) -> Result<Option<Message>, Error> {
        Ok(self.messages.pop_front())
    }
}

pub struct TopicSeparatedChannel {}

pub mod local_storage;
pub mod proxy;
pub mod spy;
pub mod topic;

use crate::{common::Error, message::Message};

pub trait ChannelWriter {
    fn send(&mut self, message: Message) -> Result<(), Error>;
}

pub trait ChannelReader {
    fn recv(&mut self) -> Result<Option<Message>, Error>;
}

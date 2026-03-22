use std::collections::{HashMap, VecDeque};

use crate::{common::Error, message::Message};

pub trait ChannelWriter {
    fn send(&mut self, message: Message) -> Result<(), Error>;
}

pub trait ChannelReader {
    fn recv(&mut self) -> Result<Option<Message>, Error>;
}

#[derive(Default)]
pub struct LocalStorageChannel {
    messages: VecDeque<Message>,
}

impl ChannelWriter for LocalStorageChannel {
    fn send(&mut self, message: Message) -> Result<(), Error> {
        self.messages.push_back(message);
        Ok(())
    }
}

impl ChannelReader for LocalStorageChannel {
    fn recv(&mut self) -> Result<Option<Message>, Error> {
        Ok(self.messages.pop_front())
    }
}

pub struct TopicSeparatedChannel {
    messages: HashMap<VecDeque<Message>>,
}

impl

#[cfg(test)]
mod test {
    use crate::{
        channel::{ChannelReader, ChannelWriter, LocalStorageChannel},
        message::Message,
    };

    #[test]
    fn test_local_storage_channel() {
        let mut ch = LocalStorageChannel::default();
        let msg_in = Message::new("default".to_string(), "1".to_string());
        ch.send(msg_in).unwrap();

        let msg_out = ch.recv().unwrap().unwrap();
        assert_eq!("1".to_string(), msg_out.json_payload);
        assert_eq!("default".to_string(), msg_out.name);
    }
}

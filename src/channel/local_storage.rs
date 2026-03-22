use std::collections::VecDeque;

use crate::{
    channel::{ChannelReader, ChannelWriter},
    common::Error,
    message::Message,
};

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

#[cfg(test)]
mod test {
    use crate::{
        channel::{ChannelReader, ChannelWriter, local_storage::LocalStorageChannel},
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

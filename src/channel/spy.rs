use crate::{
    channel::{ChannelReader, ChannelWriter},
    common::Error,
    message::Message,
};

pub struct SpyChannel<T: ChannelReader + ChannelWriter> {
    channel: T,
    spy: Box<dyn FnMut(&Message)>,
}

impl<T: ChannelReader + ChannelWriter> SpyChannel<T> {
    pub fn new<F>(channel: T, spy: F) -> Self
    where
        F: FnMut(&Message) + 'static,
    {
        Self {
            channel,
            spy: Box::new(spy),
        }
    }
}

impl<T: ChannelReader + ChannelWriter> ChannelWriter for SpyChannel<T> {
    fn send(&mut self, message: Message) -> Result<(), Error> {
        self.spy.as_mut()(&message);
        self.channel.send(message)
    }
}

impl<T: ChannelReader + ChannelWriter> ChannelReader for SpyChannel<T> {
    fn recv(&mut self) -> Result<Option<Message>, Error> {
        self.channel.recv()
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        channel::{ChannelWriter, local_storage::LocalStorageChannel, spy::SpyChannel},
        message::Message,
    };

    #[test]
    fn test_basics() {
        let memory = Rc::new(RefCell::new(Vec::new()));
        let memory_spy = memory.clone();

        let sub_ch = LocalStorageChannel::default();
        let mut ch = SpyChannel::new(sub_ch, move |msg| {
            memory_spy.borrow_mut().push(msg.name.clone());
        });

        ch.send(Message::new(String::from("a"), String::from("1")))
            .unwrap();
        ch.send(Message::new(String::from("b"), String::from("1")))
            .unwrap();

        assert_eq!(String::from("a"), memory.borrow()[0]);
        assert_eq!(String::from("b"), memory.borrow()[1]);
    }
}

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::{
    channel::{ChannelReader, ChannelWriter},
    common::Error,
    message::Message,
};

#[derive(Default)]
pub struct TopicChannel {
    messages: HashMap<String, Rc<RefCell<VecDeque<Message>>>>,
}

impl TopicChannel {
    pub fn get_reader(&mut self, topic: String) -> TopicChannelReader {
        let messages = self.messages.entry(topic).or_default();
        TopicChannelReader {
            messages: messages.clone(),
        }
    }
}

pub struct TopicChannelReader {
    messages: Rc<RefCell<VecDeque<Message>>>,
}

impl ChannelWriter for TopicChannel {
    fn send(&mut self, message: Message) -> Result<(), Error> {
        self.messages
            .entry(message.name.clone())
            .or_default()
            .borrow_mut()
            .push_back(message);
        Ok(())
    }
}

impl ChannelReader for TopicChannelReader {
    fn recv(&mut self) -> Result<Option<Message>, Error> {
        Ok(self.messages.borrow_mut().pop_front())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        channel::{ChannelReader, ChannelWriter, topic::TopicChannel},
        message::Message,
    };

    #[test]
    fn test_basics() {
        let mut wr = TopicChannel::default();
        wr.send(Message::new(String::from("rabbit"), String::from("1")))
            .unwrap();
        wr.send(Message::new(String::from("horse"), String::from("2")))
            .unwrap();

        let mut rabbit_rd = wr.get_reader(String::from("rabbit"));
        let mut horse_rd = wr.get_reader(String::from("horse"));
        let mut other_rd = wr.get_reader(String::from("other"));

        assert_eq!(None, other_rd.recv().unwrap());
        assert_eq!(
            String::from("2"),
            horse_rd.recv().unwrap().unwrap().json_payload
        );
        assert_eq!(
            String::from("1"),
            rabbit_rd.recv().unwrap().unwrap().json_payload
        );
    }
}

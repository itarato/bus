// use crate::{channel::Channel, message::Message};

// pub struct Bus {
//     channel: Box<dyn Channel>,
// }

// impl Bus {
//     pub fn new<T>(channel: Box<T>) -> Self
//     where
//         T: Channel + 'static,
//     {
//         Self { channel }
//     }

//     pub fn send(
//         &mut self,
//         topic: String,
//         json_payload: String,
//     ) -> Result<(), crate::common::Error> {
//         let message = Message::new(topic, json_payload);
//         self.channel.send(message)
//     }

//     pub fn recv(&mut self) -> Result<Option<Message>, crate::common::Error> {
//         self.channel.recv()
//     }
// }

// #[cfg(test)]
// mod test {
//     use crate::{bus::Bus, channel::LocalStorageChannel};

//     #[test]
//     fn test_basic_setup() {
//         let mut bus = Bus::new(Box::new(LocalStorageChannel::default()));
//         bus.send("default".to_string(), "1".to_string()).unwrap();

//         let msg = bus.recv().unwrap().unwrap();
//         assert_eq!("1".to_string(), msg.json_payload);
//         assert_eq!("default".to_string(), msg.name);
//     }
// }

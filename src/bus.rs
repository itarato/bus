use crate::channel::Channel;

pub struct Bus {
    channel: Box<dyn Channel>,
}

impl Bus {
    pub fn new<T>(channel: Box<T>) -> Self
    where
        T: Channel + 'static,
    {
        Self { channel }
    }
}

impl Channel for Bus {
    fn send(&mut self, json_payload: String) -> Result<(), crate::common::Error> {
        self.channel.send(json_payload)
    }

    fn recv(&mut self) -> Result<Option<String>, crate::common::Error> {
        self.channel.recv()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bus::Bus,
        channel::{Channel, LocalStorageChannel},
    };

    #[test]
    fn test_basic_setup() {
        let mut bus = Bus::new(Box::new(LocalStorageChannel::default()));
        bus.send("1".to_string()).unwrap();
        assert_eq!(Some("1".to_string()), bus.recv().unwrap());
    }
}

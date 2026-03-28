use std::{thread::spawn, time::Duration};

use bus::{
    bus::Bus,
    message::Message,
    preprocessor::{Processor, ProcessorPipeline},
    queue::{InQueue, OutQueue},
};

struct Partitioner;

impl Processor for Partitioner {
    fn process(&mut self, message: Message) -> Vec<Message> {
        if &message.name == "order" {
            vec![
                Message::new(
                    message.from.clone(),
                    Some(vec![String::from("fraud")]),
                    String::from("name"),
                    String::from("1"),
                )
                .with_chunk(bus::message::Chunk::Partial {
                    total: 2,
                    has: 1,
                    id: message.id.clone(),
                }),
                Message::new(
                    message.from.clone(),
                    Some(vec![String::from("inventory")]),
                    String::from("name"),
                    String::from("1"),
                )
                .with_chunk(bus::message::Chunk::Partial {
                    total: 2,
                    has: 1,
                    id: message.id.clone(),
                }),
            ]
        } else {
            vec![message]
        }
    }
}

struct Fraud {
    listener: OutQueue,
    publisher: InQueue,
}

impl Fraud {
    fn new(listener: OutQueue, publisher: InQueue) -> Self {
        Self {
            listener,
            publisher,
        }
    }

    fn run(self) {
        assert_eq!(
            self.listener
                .get_timeout(Duration::from_millis(10))
                .unwrap()
                .from,
            String::from("order")
        );

        self.publisher.put(Message::new(
            String::from("fraud"),
            Some(vec![String::from("logging")]),
            "order".to_string(),
            "1".to_string(),
        ));
    }
}

struct Inventory {
    listener: OutQueue,
    publisher: InQueue,
}

impl Inventory {
    fn new(listener: OutQueue, publisher: InQueue) -> Self {
        Self {
            listener,
            publisher,
        }
    }

    fn run(self) {
        assert_eq!(
            self.listener
                .get_timeout(Duration::from_millis(10))
                .unwrap()
                .from,
            String::from("order")
        );

        self.publisher.put(Message::new(
            String::from("inventory"),
            Some(vec![String::from("logging")]),
            "order".to_string(),
            "1".to_string(),
        ));
    }
}

#[test]
fn test_chunk_and_collect() {
    let mut pipeline = ProcessorPipeline::new();
    pipeline.add_processor(Partitioner);

    let mut bus = Bus::new(pipeline);

    let fraud_th = spawn({
        let listener = bus.get_listener(String::from("fraud"));
        let publisher = bus.get_publisher();
        move || Fraud::new(listener, publisher).run()
    });
    let inventory_th = spawn({
        let listener = bus.get_listener(String::from("inventory"));
        let publisher = bus.get_publisher();
        move || Fraud::new(listener, publisher).run()
    });

    let logging_recv = bus.get_listener(String::from("logging"));
    let order_send = bus.get_publisher();

    order_send.put(make_message());

    assert_eq!(logging_recv.get_timeout(Duration::from_millis(10)), None);

    fraud_th.join().unwrap();
    inventory_th.join().unwrap();

    bus.terminate();
}

fn make_message() -> Message {
    Message::new(
        String::from("order"),
        None,
        String::from("name"),
        String::from("1"),
    )
}

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, atomic::AtomicBool},
    thread::{JoinHandle, spawn},
};

use crate::{
    engine::Engine,
    preprocessor::ProcessorPipeline,
    queue::{InQueue, OutQueue, Queue},
};

pub struct Bus {
    incoming: Arc<Queue>,
    outgoing: Arc<Mutex<HashMap<String, Arc<Queue>>>>,
    thread_handle: JoinHandle<()>,
    is_terminated: Arc<AtomicBool>,
}

impl Bus {
    pub fn new(preprocessors: ProcessorPipeline) -> Self {
        let incoming = Arc::new(Queue::new());
        let outgoing = Arc::new(Mutex::new(HashMap::new()));
        let is_terminated = Arc::new(AtomicBool::new(false));

        let thread_handle = spawn({
            let incoming = incoming.clone();
            let outgoing = outgoing.clone();
            let is_terminated = is_terminated.clone();

            move || Engine::new(incoming, outgoing, preprocessors, is_terminated).run()
        });

        Self {
            incoming,
            outgoing,
            thread_handle,
            is_terminated,
        }
    }

    pub fn get_publisher(&self) -> InQueue {
        InQueue::new(self.incoming.clone())
    }

    pub fn get_listener(&mut self, name: String) -> OutQueue {
        let queue = Arc::new(Queue::new());
        let mut outgoing = self.outgoing.lock().unwrap();
        outgoing.insert(name, queue.clone());
        OutQueue::new(queue)
    }

    pub fn terminate(self) {
        self.is_terminated
            .store(true, std::sync::atomic::Ordering::Relaxed);
        self.thread_handle.join().unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{bus::Bus, message::Message, preprocessor::ProcessorPipeline};

    #[test]
    fn test_address_all() {
        let mut bus = Bus::new(ProcessorPipeline::new());

        let pub_a = bus.get_publisher();
        let pub_b = bus.get_publisher();

        let listener_a = bus.get_listener(String::from("a"));
        let listener_b = bus.get_listener(String::from("b"));

        pub_a.put(make_message());
        pub_b.put(make_message());
        pub_a.put(make_message());

        assert_eq!(listener_a.get_blocking().name, String::from("name"));
        assert_eq!(listener_a.get_blocking().name, String::from("name"));
        assert_eq!(listener_a.get_blocking().name, String::from("name"));

        assert_eq!(listener_b.get_blocking().name, String::from("name"));
        assert_eq!(listener_b.get_blocking().name, String::from("name"));
        assert_eq!(listener_b.get_blocking().name, String::from("name"));

        assert_eq!(listener_a.get(), None);
        assert_eq!(listener_b.get(), None);

        bus.terminate();
    }

    #[test]
    fn test_address_one() {
        let mut bus = Bus::new(ProcessorPipeline::new());

        let publisher = bus.get_publisher();

        let listener_a = bus.get_listener(String::from("a"));
        let listener_b = bus.get_listener(String::from("b"));
        let listener_c = bus.get_listener(String::from("c"));

        publisher.put(make_message_to(vec!["b".to_string()]));

        assert!(listener_a.get_timeout(Duration::from_millis(10)).is_none());
        assert!(listener_b.get_timeout(Duration::from_millis(10)).is_some());
        assert!(listener_c.get_timeout(Duration::from_millis(10)).is_none());

        bus.terminate();
    }

    fn make_message() -> Message {
        Message::new(
            String::from("x"),
            None,
            String::from("name"),
            vec![String::from("1")],
        )
    }

    fn make_message_to(to: Vec<String>) -> Message {
        Message::new(
            String::from("x"),
            Some(to),
            String::from("name"),
            vec![String::from("1")],
        )
    }
}

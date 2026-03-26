use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Condvar, Mutex},
    thread::{JoinHandle, spawn},
};

use crate::message::Message;

pub struct Queue {
    queue_and_condvar: Arc<(Mutex<VecDeque<Message>>, Condvar)>,
}

impl Queue {
    fn new() -> Self {
        Self {
            queue_and_condvar: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    pub fn put(&self, message: Message) {
        let (queue, cv) = &*self.queue_and_condvar;
        let mut queue = queue.lock().unwrap();

        queue.push_back(message);

        cv.notify_one();
    }

    pub fn get(&self) -> Option<Message> {
        let (queue, _cv) = &*self.queue_and_condvar;
        let mut queue = queue.lock().unwrap();
        queue.pop_front()
    }

    pub fn get_blocking(&self) -> Message {
        let (queue, cv) = &*self.queue_and_condvar;

        let mut queue = queue.lock().unwrap();

        loop {
            if let Some(message) = queue.pop_front() {
                return message;
            }

            queue = cv.wait(queue).unwrap();
        }
    }
}

pub struct InQueue {
    inner: Arc<Queue>,
}

impl InQueue {
    pub fn put(&self, message: Message) {
        self.inner.put(message);
    }
}

pub struct OutQueue {
    inner: Arc<Queue>,
}

impl OutQueue {
    pub fn get(&self) -> Option<Message> {
        self.inner.get()
    }

    pub fn get_blocking(&self) -> Message {
        self.inner.get_blocking()
    }
}

pub struct Bus {
    incoming: Arc<Queue>,
    outgoing: HashMap<String, Arc<Queue>>,
    thread_handle: JoinHandle<()>,
}

impl Bus {
    pub fn new() -> Self {
        let thread_handle = spawn(move || Self::work_thread());

        Self {
            incoming: Arc::new(Queue::new()),
            outgoing: HashMap::new(),
            thread_handle,
        }
    }

    pub fn get_publisher(&self) -> InQueue {
        InQueue {
            inner: self.incoming.clone(),
        }
    }

    pub fn get_listener(&mut self, name: String) -> OutQueue {
        let queue = Arc::new(Queue::new());
        self.outgoing.insert(name, queue.clone());
        OutQueue { inner: queue }
    }

    fn work_thread() {}
}

#[cfg(test)]
mod test {
    use crate::{bus::Bus, message::Message};

    #[test]
    fn test_non_blocking() {
        let mut bus = Bus::new();

        let pub_a = bus.get_publisher();
        let pub_b = bus.get_publisher();

        let listener_a = bus.get_listener(String::from("a"));
        let listener_b = bus.get_listener(String::from("b"));

        pub_a.put(make_message());
        pub_b.put(make_message());
        pub_a.put(make_message());
    }

    fn make_message() -> Message {
        Message::new(
            String::from("x"),
            Some(vec![String::from("y")]),
            String::from("name"),
            String::from("1"),
        )
    }
}

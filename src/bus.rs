use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Condvar, Mutex, atomic::AtomicBool},
    thread::{JoinHandle, spawn},
    time::{Duration, SystemTime},
};

use crate::message::Message;

pub struct Queue {
    queue: Arc<Mutex<VecDeque<Message>>>,
    condvar: Arc<Condvar>,
}

impl Queue {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            condvar: Arc::new(Condvar::new()),
        }
    }

    pub fn put(&self, message: Message) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(message);
        self.condvar.notify_one();
    }

    pub fn get(&self) -> Option<Message> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }

    pub fn get_blocking(&self) -> Message {
        let mut queue = self.queue.lock().unwrap();

        loop {
            if let Some(message) = queue.pop_front() {
                return message;
            }

            queue = self.condvar.wait(queue).unwrap();
        }
    }

    pub fn get_blocking_timeout(&self, mut duration: Duration) -> Option<Message> {
        let mut queue = self.queue.lock().unwrap();
        let deadline = SystemTime::now().checked_add(duration).unwrap();

        loop {
            if let Some(message) = queue.pop_front() {
                return Some(message);
            }

            duration = match deadline.duration_since(SystemTime::now()) {
                Ok(d) => d,
                Err(_) => return None,
            };
            let (new_queue, wait_res) = self.condvar.wait_timeout(queue, duration).unwrap();
            if wait_res.timed_out() {
                return None;
            }

            queue = new_queue;
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
    outgoing: Arc<Mutex<HashMap<String, Arc<Queue>>>>,
    thread_handle: JoinHandle<()>,
    is_terminated: Arc<AtomicBool>,
}

impl Bus {
    pub fn new() -> Self {
        let incoming = Arc::new(Queue::new());
        let outgoing = Arc::new(Mutex::new(HashMap::new()));
        let is_terminated = Arc::new(AtomicBool::new(false));

        let thread_handle = spawn({
            let incoming = incoming.clone();
            let outgoing = outgoing.clone();
            let is_terminated = is_terminated.clone();

            move || Self::work_thread(incoming, outgoing, is_terminated)
        });

        Self {
            incoming,
            outgoing,
            thread_handle,
            is_terminated,
        }
    }

    pub fn get_publisher(&self) -> InQueue {
        InQueue {
            inner: self.incoming.clone(),
        }
    }

    pub fn get_listener(&mut self, name: String) -> OutQueue {
        let queue = Arc::new(Queue::new());
        let mut outgoing = self.outgoing.lock().unwrap();
        outgoing.insert(name, queue.clone());
        OutQueue { inner: queue }
    }

    fn work_thread(
        incoming: Arc<Queue>,
        outgoing: Arc<Mutex<HashMap<String, Arc<Queue>>>>,
        is_terminated: Arc<AtomicBool>,
    ) {
        loop {
            if let Some(message) = incoming.get_blocking_timeout(Duration::from_millis(10)) {
                let outgoing_guard = outgoing.lock().unwrap();
                for (_, out_queue) in &*outgoing_guard {
                    out_queue.put(message.clone());
                }
            } else if is_terminated.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
        }
    }

    pub fn terminate(self) {
        self.is_terminated
            .store(true, std::sync::atomic::Ordering::Relaxed);
        self.thread_handle.join().unwrap();
    }
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

    fn make_message() -> Message {
        Message::new(
            String::from("x"),
            Some(vec![String::from("y")]),
            String::from("name"),
            String::from("1"),
        )
    }
}

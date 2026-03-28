use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, SystemTime},
};

use crate::message::Message;

pub struct Queue {
    queue: Arc<Mutex<VecDeque<Message>>>,
    condvar: Arc<Condvar>,
}

impl Queue {
    pub(crate) fn new() -> Self {
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
    pub(crate) fn new(inner: Arc<Queue>) -> Self {
        Self { inner }
    }

    pub fn put(&self, message: Message) {
        self.inner.put(message);
    }
}

pub struct OutQueue {
    inner: Arc<Queue>,
}

impl OutQueue {
    pub(crate) fn new(inner: Arc<Queue>) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> Option<Message> {
        self.inner.get()
    }

    pub fn get_blocking(&self) -> Message {
        self.inner.get_blocking()
    }
}

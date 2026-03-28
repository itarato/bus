use std::{
    collections::HashMap,
    sync::{Arc, Mutex, atomic::AtomicBool},
    time::Duration,
};

use uuid::Uuid;

use crate::{
    message::Message,
    preprocessor::{Processor, ProcessorPipeline},
    queue::Queue,
};

pub(crate) struct Engine {
    incoming: Arc<Queue>,
    outgoing: Arc<Mutex<HashMap<String, Arc<Queue>>>>,
    preprocessors: ProcessorPipeline,
    is_terminated: Arc<AtomicBool>,
}

impl Engine {
    pub(crate) fn new(
        incoming: Arc<Queue>,
        outgoing: Arc<Mutex<HashMap<String, Arc<Queue>>>>,
        preprocessors: ProcessorPipeline,
        is_terminated: Arc<AtomicBool>,
        chunks: HashMap<Uuid, Message>,
    ) -> Self {
        Self {
            incoming,
            outgoing,
            preprocessors,
            is_terminated,
        }
    }

    pub(crate) fn run(mut self) {
        loop {
            if self
                .is_terminated
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                break;
            }

            if let Some(message) = self.incoming.get_timeout(Duration::from_millis(10)) {
                // Chunk handling.
                let message = match self.manage_chunk(message) {
                    Some(m) => m,
                    None => continue,
                };

                // Preprocessor
                let processed_messages = self.preprocessors.process(message);

                for message in processed_messages {
                    let outgoing_guard = self.outgoing.lock().unwrap();

                    // Send to target.
                    match &message.to {
                        Some(list) => {
                            for target in list {
                                outgoing_guard
                                    .get(target)
                                    .map(|out_queue| out_queue.put(message.clone()));
                            }
                        }
                        None => {
                            for (_, out_queue) in &*outgoing_guard {
                                out_queue.put(message.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    fn manage_chunk(&mut self, message: Message) -> Option<Message> {
        if message.chunk.is_complete() {
            return Some(message);
        }

        unimplemented!()
    }
}

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, atomic::AtomicBool},
    time::Duration,
};

use crate::{
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
            if let Some(message) = self.incoming.get_timeout(Duration::from_millis(10)) {
                for message in self.preprocessors.process(message) {
                    let outgoing_guard = self.outgoing.lock().unwrap();

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

            if self
                .is_terminated
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                break;
            }
        }
    }
}

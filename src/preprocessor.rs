use std::collections::VecDeque;

use crate::message::Message;

pub trait Processor {
    fn process(&mut self, message: Message) -> Vec<Message>;
}

pub struct ProcessorPipeline {
    pipeline: Vec<Box<dyn Processor + Send>>,
}

impl ProcessorPipeline {
    pub fn new() -> Self {
        Self {
            pipeline: Vec::new(),
        }
    }

    pub fn add_processor<P>(&mut self, processor: P)
    where
        P: Processor + Send + 'static,
    {
        self.pipeline.push(Box::new(processor));
    }
}

impl Processor for ProcessorPipeline {
    fn process(&mut self, message: Message) -> Vec<Message> {
        let mut out = VecDeque::new();
        out.push_back(message);

        for step in self.pipeline.iter_mut() {
            let mut processed = VecDeque::new();
            while let Some(msg) = out.pop_front() {
                for processed_msg in step.process(msg) {
                    processed.push_back(processed_msg);
                }
            }

            out.append(&mut processed);
        }

        out.into_iter().collect()
    }
}

use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{JoinHandle, spawn},
};

use crate::{
    channel::{ChannelReader, ChannelWriter, local_storage::LocalStorageChannel},
    common::Error,
};

pub struct ProxyChannel {
    incoming: Arc<(Mutex<LocalStorageChannel>, Condvar)>,
    outgoing: Arc<(Mutex<LocalStorageChannel>, Condvar)>,
    th: JoinHandle<()>,
}

impl ProxyChannel {
    pub fn new<T>(channel: T) -> Self
    where
        T: ChannelReader + ChannelWriter + Send + 'static,
    {
        let incoming = Arc::new((Mutex::new(LocalStorageChannel::default()), Condvar::new()));
        let outgoing = Arc::new((Mutex::new(LocalStorageChannel::default()), Condvar::new()));
        let th = spawn({
            let incoming = incoming.clone();
            let outgoing = outgoing.clone();
            move || Self::runtime(incoming, outgoing, channel)
        });

        Self {
            incoming,
            outgoing,
            th,
        }
    }

    fn runtime<T>(
        incoming: Arc<(Mutex<LocalStorageChannel>, Condvar)>,
        outgoing: Arc<(Mutex<LocalStorageChannel>, Condvar)>,
        channel: T,
    ) where
        T: ChannelReader + ChannelWriter + 'static,
    {
        loop {
            let (in_msgs_mx, in_cv) = &*incoming;
            let mut in_msgs = in_msgs_mx.lock().unwrap();

            loop {
                if let Some(msg) = in_msgs.recv().unwrap() {
                    let

                    let (out_msgs_mx, out_cv) = &*outgoing;

                    break;
                }

                in_msgs = in_cv.wait(in_msgs).unwrap();
            }
        }
    }
}

impl ChannelWriter for ProxyChannel {
    fn send(&mut self, message: crate::message::Message) -> Result<(), Error> {
        let (msgs_mx, cv) = &*self.incoming;
        let mut msgs = msgs_mx.lock().unwrap();

        let result = msgs.send(message);
        cv.notify_one();
        result
    }
}

impl ChannelReader for ProxyChannel {
    fn recv(&mut self) -> Result<Option<crate::message::Message>, Error> {
        let (msgs_mx, cv) = &*self.outgoing;
        let mut msgs = msgs_mx.lock().unwrap();
        msgs.recv()
    }
}

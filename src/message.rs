use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    name: String,
    payload: Vec<u8>,
}

impl Message {
    pub fn new(name: String, payload: Vec<u8>) -> Self {
        Self {
            // TODO: Context likely needs to be persisted statically (for sub nanosec sequence gen).
            id: Uuid::new_v7(Timestamp::now(uuid::ContextV7::new())),
            name,
            payload,
        }
    }
}

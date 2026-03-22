use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub name: String,
    pub json_payload: String,
}

impl Message {
    pub fn new(name: String, json_payload: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            json_payload,
        }
    }
}

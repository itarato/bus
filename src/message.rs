use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub from: String,
    pub to: Option<Vec<String>>,
    pub name: String,
    pub json_payload: String,
}

impl Message {
    pub fn new(from: String, to: Option<Vec<String>>, name: String, json_payload: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            name,
            json_payload,
        }
    }
}

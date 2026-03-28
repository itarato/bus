use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Chunk {
    Full,
    Partial { total: usize, has: usize, id: Uuid },
}

impl Chunk {
    pub(crate) fn is_complete(&self) -> bool {
        match self {
            Self::Full => true,
            Self::Partial { total, has, .. } => total == has,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Message {
    pub id: Uuid,
    pub from: String,
    pub to: Option<Vec<String>>,
    pub name: String,
    pub json_payload: String,
    pub chunk: Chunk,
}

impl Message {
    pub fn new(from: String, to: Option<Vec<String>>, name: String, json_payload: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            name,
            json_payload,
            chunk: Chunk::Full,
        }
    }

    pub fn with_chunk(mut self, chunk: Chunk) -> Self {
        self.chunk = chunk;
        self
    }
}

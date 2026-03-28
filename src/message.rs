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

    fn is_compatible(&self, other: &Chunk) -> bool {
        match (self, other) {
            (
                Self::Partial {
                    total: self_total,
                    has: self_has,
                    id: self_id,
                },
                Self::Partial {
                    total: other_total,
                    has: other_has,
                    id: other_id,
                },
            ) => {
                self_total == other_total
                    && self_has + other_has <= *self_total
                    && self_id == other_id
            }
            _ => false,
        }
    }

    fn merge(&mut self, other: &Chunk) {
        assert!(self.is_compatible(other));
        match (self, other) {
            (Self::Partial { has: self_has, .. }, Self::Partial { has: other_has, .. }) => {
                *self_has += other_has;
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Message {
    pub id: Uuid,
    pub from: String,
    pub to: Option<Vec<String>>,
    pub name: String,
    pub json_payloads: Vec<String>,
    pub chunk: Chunk,
}

impl Message {
    pub fn new(
        from: String,
        to: Option<Vec<String>>,
        name: String,
        json_payloads: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            name,
            json_payloads,
            chunk: Chunk::Full,
        }
    }

    pub fn with_chunk(mut self, chunk: Chunk) -> Self {
        self.chunk = chunk;
        self
    }

    pub(crate) fn merge_chunk(mut self, other: &Message) -> Self {
        self.chunk.merge(&other.chunk);
        self.json_payloads.extend_from_slice(&other.json_payloads);
        self
    }
}

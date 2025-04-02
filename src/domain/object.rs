use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub id: String,
    pub key: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub size: usize,
}

impl Object {
    pub fn new(key: String, content: Vec<u8>, content_type: String) -> Self {
        let size = content.len();
        Self {
            id: Uuid::new_v4().to_string(),
            key,
            content,
            content_type,
            size,
        }
    }
}
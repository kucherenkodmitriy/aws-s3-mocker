use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub id: Uuid,
    pub key: String,
    pub bucket_id: Uuid,
    pub data: Vec<u8>,
}

impl Object {
    pub fn new(key: String, bucket_id: Uuid, data: Vec<u8>) -> Object {
        Object {
            id: Uuid::new_v4(),
            key,
            bucket_id,
            data,
        }
    }
}
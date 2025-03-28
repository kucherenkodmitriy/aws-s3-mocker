use thiserror::Error;

pub enum DomainError {
    #[error("Bucket not found: {0}")]
    BucketNotFound(String),
    #[error("Object not found: {0}")]
    ObjectNotFound(String),
    #[error("Storage error: {0}")]
    StorageError(String),
}
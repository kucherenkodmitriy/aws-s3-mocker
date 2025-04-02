use std::io;
use crate::domain::bucket::Bucket;
use crate::domain::object::Object;

#[async_trait::async_trait]
pub trait BucketStorage: Send + Sync {
    async fn create_bucket(&self, bucket: &Bucket) -> io::Result<()>;
    async fn list_buckets(&self) -> io::Result<Vec<String>>;
    async fn delete_bucket(&self, bucket_name: &str) -> io::Result<()>;
}

#[async_trait::async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn put_object(&self, bucket_name: &str, object: &Object) -> io::Result<()>;
    async fn get_object(&self, bucket_name: &str, key: &str) -> io::Result<Object>;
    async fn delete_object(&self, bucket_name: &str, key: &str) -> io::Result<()>;
    async fn list_objects(&self, bucket_name: &str) -> io::Result<Vec<String>>;
}

// Combined trait for implementations that support both
#[async_trait::async_trait]
pub trait Storage: BucketStorage + ObjectStorage {} 
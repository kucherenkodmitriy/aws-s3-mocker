use std::fs;
use std::path::PathBuf;
use std::io::{self, Write, Read};
use crate::domain::bucket::Bucket;
use crate::domain::object::Object;
mod traits;
pub use traits::{Storage, BucketStorage, ObjectStorage};

pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

#[async_trait::async_trait]
impl BucketStorage for FileStorage {
    async fn create_bucket(&self, bucket: &Bucket) -> io::Result<()> {
        let bucket_path = self.base_path.join(&bucket.name);
        fs::create_dir_all(&bucket_path)?;
        Ok(())
    }

    async fn list_buckets(&self) -> io::Result<Vec<String>> {
        let mut buckets = Vec::new();
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    buckets.push(name.to_string());
                }
            }
        }
        Ok(buckets)
    }

    async fn delete_bucket(&self, bucket_name: &str) -> io::Result<()> {
        let bucket_path = self.base_path.join(bucket_name);
        if bucket_path.exists() {
            fs::remove_dir_all(bucket_path)?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl ObjectStorage for FileStorage {
    async fn put_object(&self, bucket_name: &str, object: &Object) -> io::Result<()> {
        let object_path = self.base_path.join(bucket_name).join(&object.key);
        if let Some(parent) = object_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(object_path)?;
        file.write_all(&object.content)?;
        Ok(())
    }

    async fn get_object(&self, bucket_name: &str, key: &str) -> io::Result<Object> {
        let object_path = self.base_path.join(bucket_name).join(key);
        let mut content = Vec::new();
        let mut file = fs::File::open(object_path)?;
        file.read_to_end(&mut content)?;
        
        // In a real implementation, we would store and retrieve metadata
        Ok(Object::new(
            key.to_string(),
            content,
            "application/octet-stream".to_string(),
        ))
    }

    async fn delete_object(&self, bucket_name: &str, key: &str) -> io::Result<()> {
        let object_path = self.base_path.join(bucket_name).join(key);
        if object_path.exists() {
            fs::remove_file(object_path)?;
        }
        Ok(())
    }

    async fn list_objects(&self, bucket_name: &str) -> io::Result<Vec<String>> {
        let bucket_path = self.base_path.join(bucket_name);
        let mut objects = Vec::new();
        for entry in fs::read_dir(bucket_path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    objects.push(name.to_string());
                }
            }
        }
        Ok(objects)
    }
}

#[async_trait::async_trait]
impl Storage for FileStorage {} 
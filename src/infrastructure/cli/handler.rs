use std::path::PathBuf;
use crate::domain::bucket::Bucket;
use crate::domain::object::Object;
use crate::infrastructure::storage::{FileStorage, Storage};

pub struct CliHandler {
    storage: Box<dyn Storage>,
}

impl CliHandler {
    pub fn new() -> Self {
        let base_path = PathBuf::from("./s3-data");
        Self {
            storage: Box::new(FileStorage::new(base_path)),
        }
    }

    pub async fn create_bucket(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let bucket = Bucket::new(name.to_string());
        self.storage.create_bucket(&bucket).await?;
        println!("Created bucket: {}", name);
        Ok(())
    }

    pub async fn list_buckets(&self) -> Result<(), Box<dyn std::error::Error>> {
        let buckets = self.storage.list_buckets().await?;
        if buckets.is_empty() {
            println!("No buckets found");
            return Ok(());
        }
        println!("Buckets:");
        for bucket in buckets {
            println!("- {}", bucket);
        }
        Ok(())
    }

    pub async fn delete_bucket(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.storage.delete_bucket(name).await?;
        println!("Deleted bucket: {}", name);
        Ok(())
    }

    pub async fn put_object(&self, bucket_name: &str, key: &str, content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let object = Object::new(key.to_string(), content, "application/octet-stream".to_string());
        self.storage.put_object(bucket_name, &object).await?;
        println!("Put object: {} in bucket: {}", key, bucket_name);
        Ok(())
    }

    pub async fn get_object(&self, bucket_name: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let object = self.storage.get_object(bucket_name, key).await?;
        match std::str::from_utf8(&object.content) {
            Ok(content) => println!("Object content: {}", content),
            Err(_) => println!("Object content is binary (length: {} bytes)", object.content.len()),
        }
        Ok(())
    }

    pub async fn delete_object(&self, bucket_name: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.storage.delete_object(bucket_name, key).await?;
        println!("Deleted object: {} from bucket: {}", key, bucket_name);
        Ok(())
    }

    pub async fn list_objects(&self, bucket_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let objects = self.storage.list_objects(bucket_name).await?;
        if objects.is_empty() {
            println!("No objects found in bucket: {}", bucket_name);
            return Ok(());
        }
        println!("Objects in bucket {}:", bucket_name);
        for object in objects {
            println!("- {}", object);
        }
        Ok(())
    }
} 
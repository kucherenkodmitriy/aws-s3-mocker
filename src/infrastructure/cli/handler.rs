use std::sync::{Arc, Mutex};
use crate::domain::bucket::Bucket;

pub struct CliHandler {
    buckets: Arc<Mutex<Vec<Bucket>>>,
}

impl CliHandler {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn create_bucket(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buckets = self.buckets.lock().map_err(|_| "Failed to lock buckets")?;
        let bucket = Bucket::new(name.to_string());
        buckets.push(bucket);
        println!("Created bucket: {}", name);
        Ok(())
    }

    pub fn list_buckets(&self) -> Result<(), Box<dyn std::error::Error>> {
        let buckets = self.buckets.lock().map_err(|_| "Failed to lock buckets")?;
        if buckets.is_empty() {
            println!("No buckets found");
            return Ok(());
        }
        println!("Buckets:");
        for bucket in buckets.iter() {
            println!("- {}", bucket.name);
        }
        Ok(())
    }

    pub fn delete_bucket(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buckets = self.buckets.lock().map_err(|_| "Failed to lock buckets")?;
        if let Some(pos) = buckets.iter().position(|b| b.name == name) {
            buckets.remove(pos);
            println!("Deleted bucket: {}", name);
        } else {
            println!("Bucket not found: {}", name);
        }
        Ok(())
    }
} 
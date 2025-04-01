use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub id: String,
    pub name: String,
}

impl Bucket {
    pub fn new(name: String) -> Self {
        if !Self::validate_name(&name) {
            panic!("Invalid bucket name: {}", name);
        }
        Self {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }

    pub fn validate_name(name: &str) -> bool {
        // S3 bucket name rules:
        // 1. Must be between 3 and 63 characters long
        // 2. Can only contain lowercase letters, numbers, dots (.), and hyphens (-)
        // 3. Must begin and end with a letter or number
        // 4. Must not be formatted as an IP address
        if name.len() < 3 || name.len() > 63 {
            return false;
        }

        // Check if starts and ends with letter or number
        if !name.chars().next().unwrap_or('a').is_alphanumeric() ||
           !name.chars().last().unwrap_or('a').is_alphanumeric() {
            return false;
        }

        // Check if contains only valid characters
        for c in name.chars() {
            if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '.' && c != '-' {
                return false;
            }
        }

        // Check if it's not an IP address
        if name.split('.').all(|part| part.parse::<u8>().is_ok()) {
            return false;
        }

        true
    }

    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        Self::validate_name(&self.name)
    }

    #[allow(dead_code)]
    pub fn with_name(mut self, name: String) -> Self {
        if !Self::validate_name(&name) {
            panic!("Invalid bucket name: {}", name);
        }
        self.name = name;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_bucket_creation() {
        let bucket_name = "test-bucket".to_string();
        let bucket = Bucket::new(bucket_name.clone());

        assert_eq!(bucket.name, bucket_name);
        assert!(bucket.id != Uuid::nil().to_string());
    }

    #[test]
    fn test_bucket_serialization() {
        let bucket = Bucket::new("test-bucket".to_string());
        let serialized = serde_json::to_string(&bucket).unwrap();
        let deserialized: Bucket = serde_json::from_str(&serialized).unwrap();

        assert_eq!(bucket.name, deserialized.name);
        assert_eq!(bucket.id, deserialized.id);
    }

    #[test]
    fn test_bucket_clone() {
        let bucket = Bucket::new("test-bucket".to_string());
        let cloned = bucket.clone();

        assert_eq!(bucket.name, cloned.name);
        assert_eq!(bucket.id, cloned.id);
    }

    #[test]
    fn test_bucket_debug() {
        let bucket = Bucket::new("test-bucket".to_string());
        let debug_string = format!("{:?}", bucket);

        assert!(debug_string.contains("test-bucket"));
        assert!(debug_string.contains(&bucket.id));
    }

    #[test]
    fn test_bucket_name_validation() {
        // Valid names
        assert!(Bucket::validate_name("my-bucket"), "my-bucket should be valid");
        assert!(Bucket::validate_name("my.bucket"), "my.bucket should be valid");
        assert!(Bucket::validate_name("mybucket123"), "mybucket123 should be valid");
        assert!(Bucket::validate_name("my-bucket-123"), "my-bucket-123 should be valid");
        assert!(Bucket::validate_name("my.bucket.123"), "my.bucket.123 should be valid");
        assert!(Bucket::validate_name("my-bucket.123"), "my-bucket.123 should be valid");

        // Invalid names
        assert!(!Bucket::validate_name(""), "empty string should be invalid");
        assert!(!Bucket::validate_name("a"), "single character should be invalid");
        assert!(!Bucket::validate_name("a".repeat(64).as_str()), "64+ characters should be invalid");
        assert!(!Bucket::validate_name("-my-bucket"), "starting with hyphen should be invalid");
        assert!(!Bucket::validate_name("my-bucket-"), "ending with hyphen should be invalid");
        
        // IP address format tests
        assert!(!Bucket::validate_name("192.168.1.1"), "IP address should be invalid");
        assert!(!Bucket::validate_name("10.0.0.0"), "IP address should be invalid");
        assert!(!Bucket::validate_name("172.16.0.0"), "IP address should be invalid");
        
        // Edge cases
        assert!(Bucket::validate_name("my.bucket.123.456"), "more than 4 parts should be valid");
        assert!(Bucket::validate_name("my-bucket.123.456"), "mixed separators should be valid");
        assert!(Bucket::validate_name("my.bucket-123"), "mixed separators should be valid");
    }

    #[test]
    fn test_bucket_with_name() {
        let mut bucket = Bucket::new("old-name".to_string());
        let new_name = "new-name".to_string();
        
        bucket = bucket.with_name(new_name.clone());
        assert_eq!(bucket.name, new_name);
    }

    #[test]
    fn test_bucket_is_valid() {
        let valid_bucket = Bucket::new("valid-bucket".to_string());
        assert!(valid_bucket.is_valid());

        // Create a bucket with an invalid name by directly constructing it
        let invalid_bucket = Bucket {
            id: Uuid::new_v4().to_string(),
            name: "-invalid-bucket".to_string(),
        };
        assert!(!invalid_bucket.is_valid());
    }
}
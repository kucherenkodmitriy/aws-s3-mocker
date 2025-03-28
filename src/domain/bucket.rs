use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub id: Uuid,
    pub name: String,
}

impl Bucket {
    pub fn new(name: String) -> Bucket {
        Bucket {
            id: Uuid::new_v4(),
            name,
        }
    }

    pub fn validate_name(name: &str) -> bool {
        // S3 bucket name rules:
        // - 3-63 characters long
        // - Can contain lowercase letters, numbers, dots (.), and hyphens (-)
        // - Must start and end with a letter or number
        // - Must not be formatted as an IP address
        if name.len() < 3 || name.len() > 63 {
            return false;
        }

        // Check if starts and ends with letter or number
        if !name.chars().next().unwrap_or('a').is_alphanumeric() ||
           !name.chars().last().unwrap_or('a').is_alphanumeric() {
            return false;
        }

        // Check if contains only valid characters
        if !name.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
            return false;
        }

        // Check if it's an IP address format (e.g., 192.168.1.1)
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() == 4 {
            // Check if all parts are valid numbers
            if parts.iter().all(|part| part.parse::<u8>().is_ok()) {
                return false;
            }
        }

        true
    }

    pub fn is_valid(&self) -> bool {
        Self::validate_name(&self.name)
    }

    pub fn with_name(mut self, name: String) -> Self {
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
        assert!(bucket.id != Uuid::nil());
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
        assert!(debug_string.contains(&bucket.id.to_string()));
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

        let invalid_bucket = Bucket::new("-invalid-bucket".to_string());
        assert!(!invalid_bucket.is_valid());
    }
}
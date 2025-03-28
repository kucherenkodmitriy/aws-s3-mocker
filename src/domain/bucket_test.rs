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
} 
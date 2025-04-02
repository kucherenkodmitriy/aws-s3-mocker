use std::sync::Arc;
use std::path::PathBuf;
use crate::domain::bucket::Bucket;
use crate::infrastructure::storage::{FileStorage, Storage};

pub struct AppState {
    storage: Box<dyn Storage>,
}

impl AppState {
    pub fn new() -> Self {
        let base_path = PathBuf::from("./s3-data");
        Self {
            storage: Box::new(FileStorage::new(base_path)),
        }
    }
}

pub async fn create_bucket(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    axum::extract::Json(payload): axum::extract::Json<CreateBucketRequest>,
) -> Result<axum::Json<Bucket>, axum::http::StatusCode> {
    let bucket = Bucket::new(payload.name);
    state.storage.create_bucket(&bucket).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(bucket))
}

pub async fn list_buckets(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> Result<axum::Json<Vec<String>>, axum::http::StatusCode> {
    let buckets = state.storage.list_buckets().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(buckets))
}

pub async fn delete_bucket(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<(), axum::http::StatusCode> {
    state.storage.delete_bucket(&name).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct CreateBucketRequest {
    name: String,
} 
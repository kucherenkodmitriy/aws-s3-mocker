use axum::{Router, routing::{get, put}, extract::{Path, State}, Json};
use std::sync::{Arc, Mutex};
use crate::domain::bucket::Bucket;

#[derive(Clone)]
pub struct AppState {
    pub buckets: Arc<Mutex<Vec<Bucket>>>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // .route("/", get(get_buckets))
        .route("/{bucket}", get(get_bucket))
        .route("/{bucket}", put(create_bucket))
        // .route("/{bucket}/{object}", get(get_object))
        // .route("/{bucket}/{object}", put(create_object))
        .with_state(state)
}

async fn get_bucket(State(state): State<AppState>, Path(bucket_name): Path<String>) -> Json<Option<Bucket>> {
    let buckets = state.buckets.lock().unwrap();
    Json(buckets.iter().find(|b| b.name == bucket_name).cloned())
}

async fn create_bucket(State(state): State<AppState>, Path(bucket_name): Path<String>) -> Json<Bucket> {
    let bucket = Bucket::new(bucket_name);
    state.buckets.lock().unwrap().push(bucket.clone());
    Json(bucket)
}
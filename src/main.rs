mod domain;
mod infrastructure;

use infrastructure::api::{create_router, AppState};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::signal;

#[tokio::main]
async fn main() {
    let state: AppState = AppState {
        buckets: Arc::new(Mutex::new(vec![])),
    };

    let app = create_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
    println!("Shutting down...");
}

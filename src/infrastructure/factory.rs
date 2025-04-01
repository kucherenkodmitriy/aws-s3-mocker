use std::sync::{Arc, Mutex};
use crate::infrastructure::api::{create_router, AppState};
use crate::infrastructure::cli::CliHandler;

#[async_trait::async_trait]
pub trait ApplicationFactory {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct ApiFactory {
    port: u16,
}

pub struct CliFactory {
    command: String,
    args: Vec<String>,
}

impl ApiFactory {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

impl CliFactory {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }
}

#[async_trait::async_trait]
impl ApplicationFactory for ApiFactory {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let state = AppState {
            buckets: Arc::new(Mutex::new(vec![])),
        };

        let app = create_router(state);
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = tokio::net::TcpListener::bind(addr).await?;

        println!("Server running on {}", addr);
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl ApplicationFactory for CliFactory {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cli = CliHandler::new();
        
        match self.command.as_str() {
            "create" => {
                if self.args.is_empty() {
                    return Err("Bucket name is required".into());
                }
                let bucket_name = &self.args[0];
                cli.create_bucket(bucket_name)?;
            }
            "list" => {
                cli.list_buckets()?;
            }
            "delete" => {
                if self.args.is_empty() {
                    return Err("Bucket name is required".into());
                }
                let bucket_name = &self.args[0];
                cli.delete_bucket(bucket_name)?;
            }
            _ => {
                return Err(format!("Unknown command: {}", self.command).into());
            }
        }
        Ok(())
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
    println!("Shutting down...");
} 
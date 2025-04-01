mod domain;
mod infrastructure;

use infrastructure::{ApplicationFactory, ApiFactory, CliFactory};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let factory: Box<dyn ApplicationFactory> = if args.len() > 1 {
        // CLI mode
        let command = args[1].clone();
        let command_args = args[2..].to_vec();
        Box::new(CliFactory::new(command, command_args))
    } else {
        // API mode
        Box::new(ApiFactory::new(3000))
    };

    factory.run().await
}

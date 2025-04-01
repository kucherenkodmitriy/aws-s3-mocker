# AWS S3 Mocker (Work In Progress)

A lightweight mock implementation of AWS S3 API for local development and testing purposes. This project provides a simple HTTP server that mimics basic S3 operations.

## Features

- Create buckets
- Get bucket information
- RESTful API interface
- Graceful shutdown support

## Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```bash
   git https://github.com/kucherenkodmitriy/aws-s3-mocker.git
   cd AwsS3Mocker
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Running the Server

Start the server with:
```bash
cargo run
```

The server will start on `http://localhost:3000`.

## API Endpoints

### Bucket Operations

- `PUT /{bucket}` - Create a new bucket
- `GET /{bucket}` - Get bucket information

## Testing

The project includes a set of bash scripts for testing the API. See the [scripts documentation](scripts/README.md) for more information.

### Quick Test Example

1. Start the server:
   ```bash
   cargo run
   ```

2. In another terminal, create and test a bucket:
   ```bash
   cd scripts
   ./test_bucket_operations.sh my-test-bucket
   ```

## Project Structure

```
AwsS3Mocker/
├── src/
│   ├── domain/         # Domain models and business logic
│   ├── infrastructure/ # API implementation and external interfaces
│   └── main.rs         # Application entry point
├── scripts/            # Test scripts
└── Cargo.toml         # Project dependencies and configuration
```

## Development

### Adding New Features

1. Add new endpoints in `src/infrastructure/api/api.rs`
2. Implement corresponding handlers
3. Update the router configuration
4. Add test scripts if needed

### Running Tests

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Inspired by AWS S3 API 

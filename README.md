# Survey Application

A Rust-based survey management system.

## Project Structure

```
survey/
├── src/
│   ├── models/          # Data models and entities
│   ├── services/        # Business logic layer
│   ├── controllers/     # HTTP request controllers
│   ├── repository/      # Data access layer
│   ├── middleware/      # Request/response middleware
│   ├── utils/           # Utility functions
│   ├── config/          # Application configuration
│   ├── handlers/        # HTTP route handlers
│   ├── dto/             # Data Transfer Objects
│   ├── errors/          # Custom error types
│   └── main.rs          # Application entry point
├── tests/               # Integration and unit tests
├── docs/                # Documentation
├── examples/            # Usage examples
├── migrations/          # Database migrations
├── scripts/             # Build and deployment scripts
├── assets/              # Static assets
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

## Features

- Survey creation and management
- Question types support
- Response collection and analysis
- User management
- RESTful API

## Getting Started

1. Clone the repository
2. Install Rust and Cargo
3. Run `cargo build` to build the project
4. Run `cargo run` to start the application

## Development

- `cargo test` - Run tests
- `cargo fmt` - Format code
- `cargo clippy` - Run linter

## License

MIT License

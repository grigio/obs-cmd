# obs-cmd Guide

## Technology Stack
- **Rust 2021 Edition**: Command-line tool for OBS Studio control
- **obws**: OBS WebSocket client library
- **clap**: Command-line argument parsing
- **tokio**: Async runtime

## Project Structure
```
src/
├── main.rs          # Application entry point and WebSocket connection
├── cli.rs           # CLI argument definitions and parsing
└── handler.rs       # Command handling and OBS API calls
```

## Development Commands
```bash
# Build and run
cargo run                    # Build and run locally
cargo build --release        # Release build

# Code quality
cargo fmt                    # Format code
cargo clippy -- -D warnings  # Lint with strict warnings
cargo test                   # Run tests

# Installation
cargo install --path .       # Install locally
```

## Key Components

### CLI Structure (cli.rs)
- Defines all OBS commands using clap subcommands
- Handles WebSocket URL parsing and validation
- Supports environment variable `OBS_WEBSOCKET_URL` for connection

### Command Handler (handler.rs)
- Implements all OBS operations via obws client
- Handles scenes, recording, streaming, audio, filters, and more
- Provides error handling and user feedback

### Main Entry (main.rs)
- Establishes WebSocket connection to OBS
- Supports both CLI flag and environment variable configuration
- Routes commands to handler

## Common Patterns
- Async/await for all WebSocket operations
- Result-based error handling throughout
- Subcommand structure for logical command grouping
- Environment variable fallback for configuration
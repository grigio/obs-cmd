# obs-cmd Guide

## Notes

Keep AGENTS.md updated with non-trivial project design, keep it minimal and essetial

## Technology Stack
- **Rust 2021 Edition**: Command-line tool for OBS Studio control
- **obws**: OBS WebSocket client library
- **clap**: Command-line argument parsing with derive macros
- **tokio**: Async runtime with multi-threaded scheduler
- **thiserror**: Error handling with derive macros
- **async-trait**: Async trait support for command handlers
- The official obs-websocket spec https://raw.githubusercontent.com/obsproject/obs-websocket/master/docs/generated/protocol.md

## Project Structure
```
src/
├── main.rs              # Entry point with connection setup
├── cli.rs               # CLI argument parsing and command definitions
├── connection.rs        # WebSocket connection management with retry logic
├── handler.rs           # Command dispatcher routing to handlers
├── error.rs             # Comprehensive error type definitions
└── handlers/            # Modular command handlers
    ├── mod.rs           # CommandHandler trait and utilities
    ├── general.rs       # Info and hotkey commands
    ├── scenes.rs        # Scene management
    ├── recording.rs     # Recording control
    ├── streaming.rs     # Streaming control
    ├── audio.rs         # Audio source management
    ├── filters.rs       # Filter management
    ├── media.rs         # Media input control
    ├── sources.rs       # Source screenshots
    ├── ui.rs            # Projector management
    └── ...              # Other specialized handlers
```

## Architecture Patterns

### Command Handler Pattern
- All OBS operations implement `CommandHandler` trait in `src/handlers/mod.rs`
- Trait provides `execute()` and `description()` methods for consistent interface
- Handlers are boxed and dispatched in `src/handler.rs`

### Connection Management
- Retry logic with configurable timeouts in `src/connection.rs`
- Connection health checks before command execution in `src/handler.rs`
- Support for both CLI args and `OBS_WEBSOCKET_URL` environment variable

### Error Handling
- Comprehensive error types in `src/error.rs` using thiserror
- Result type alias for consistent error handling
- Detailed error messages with actionable guidance

### CLI Design
- Custom URL parsing for `obws://hostname:port/password` format in `src/cli.rs`
- Subcommand structure for logical grouping (scenes, recording, streaming, etc.)
- Duration parsing for media controls in `src/cli.rs`

## Development Commands
```bash
# Build and run
cargo run                    # Build and run locally
cargo build --release        # Release build with optimizations

# Code quality
cargo fmt                    # Format code
cargo clippy -- -D warnings  # Lint with strict warnings
cargo test                   # Run tests

# Dependency management
cargo update                 # Update dependencies
cargo audit                  # Security audit
cargo deny check             # License and dependency checks

# Installation
cargo install --path .       # Install locally
```

## Release Configuration
- Optimized release profile in `Cargo.toml` with size optimizations
- LTO enabled, single codegen unit, panic=abort for smaller binaries
- Strip symbols for reduced binary size

## Security & Quality
- License compliance via deny.toml
- GitHub workflows for automated testing and security scanning
- Branch protection requiring workflow checks
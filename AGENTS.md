# obs-cmd Guide

## Notes

Keep AGENTS.md updated with non-trivial project design for developers, keep it minimal and essential. Update README.md instead for user related info.

## Technology Stack
- **Rust 2021 Edition**: Command-line tool for OBS Studio control
- **obws**: OBS WebSocket client library. https://docs.rs/obws/latest/obws/ 
- **clap**: Command-line argument parsing with derive macros
- **tokio**: Async runtime with multi-threaded scheduler
- **thiserror**: Error handling with derive macros
- **async-trait**: Async trait support for command handlers
- **serde_json**: JSON serialization for stream service settings
- The obs-websocket spec obs-websocket-v5.spec.md

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
    ├── config.rs        # Config management (profiles, video, stream, record)
    ├── general.rs       # Info and hotkey commands
    ├── scenes.rs        # Scene management
    ├── recording.rs     # Recording control
    ├── streaming.rs     # Streaming control
    ├── audio.rs         # Audio source management
    ├── filters.rs       # Filter management
    ├── inputs.rs        # Input management (CRUD, settings, audio controls)
    ├── media.rs         # Media input control
    ├── sources.rs       # Source screenshots
    ├── ui.rs            # Projector management
    ├── scene_collections.rs # Scene collection management
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
- Subcommand structure for logical grouping (scenes, recording, streaming, config, etc.)
- Duration parsing for media controls in `src/cli.rs`
- Recording chapter creation with optional naming support
- Config management commands for profiles, video settings, stream service, and record directory

### Recording Control
- Start, stop, toggle recording with status monitoring
- Pause, resume recording functionality
- Recording status queries and validation
- Chapter creation support for Hybrid MP4 recordings (OBS 30.0+, WebSocket v5.5.0+)

### Input Management Features
- **CRUD Operations**: List, create, remove, rename inputs
- **Input Settings**: Get/set input configurations and default settings
- **Audio Controls**: Volume, mute/unmute/toggle, balance, sync offset
- **Advanced Audio**: Monitor type configuration, audio track management
- **Input Discovery**: List input kinds and special inputs

### Config Management Features
- **Profile Management**: List, create, remove, switch between OBS profiles
- **Video Settings**: Get/set base resolution, output resolution, FPS settings
- **Stream Service**: Get/set streaming service configuration (RTMP server, key)
- **Record Directory**: Get/set the directory where recordings are saved
- **Scene Collections**: List, create, switch between scene collections

## Development Commands

### Traditional Rust Development
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

### Nix Development
```bash
# Nix development (alternative)
nix develop                  # Enter development shell
nix build                    # Build reproducible package
nix flake check             # Validate flake configuration

# Run from Nix store
./result/bin/obs-cmd --help

# Code quality (within nix develop)
cargo fmt                    # Format code
cargo clippy -- -D warnings  # Lint with strict warnings
cargo test                   # Run tests
```

## Release Configuration
- Optimized release profile in `Cargo.toml` with size optimizations
- LTO enabled, single codegen unit, panic=abort for smaller binaries
- Strip symbols for reduced binary size

## Security & Quality
- License compliance via deny.toml
- GitHub workflows for automated testing and security scanning
- Branch protection requiring workflow checks
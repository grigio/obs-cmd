use thiserror::Error;

/// Error types for obs-cmd operations.
///
/// This enum represents all possible errors that can occur during
/// OBS WebSocket operations, connection handling, and command execution.
/// Each error variant provides detailed, actionable error messages.
#[derive(Error, Debug)]
#[allow(clippy::result_large_err)]
pub enum ObsCmdError {
    #[error(
        "WebSocket connection failed: {0}. Ensure OBS is running with WebSocket server enabled"
    )]
    ConnectionError(#[from] obws::error::Error),

    #[error("Invalid URL format: {0}. Use format: obsws://hostname:port/password")]
    UrlParseError(#[from] url::ParseError),

    #[error("Environment variable error: {0}. Set OBS_WEBSOCKET_URL environment variable")]
    EnvError(#[from] std::env::VarError),

    #[error("Invalid audio command '{command}'. Valid commands are: mute, unmute, toggle, status")]
    InvalidAudioCommand { command: String },

    #[error("Invalid filter command '{command}'. Valid commands are: enable, disable, toggle")]
    InvalidFilterCommand { command: String },

    #[error("Invalid scene item command '{command}'. Valid commands are: enable, disable, toggle")]
    InvalidSceneItemCommand { command: String },

    #[error("Monitor index {index} is not available. Check available monitors with OBS")]
    MonitorNotAvailable { index: u32 },

    #[error("Recording is not currently active. Start recording first")]
    RecordingNotActive,

    #[error("Recording is currently paused. Use resume command to continue")]
    RecordingPaused,

    #[error("No replay buffer recording found. Start replay buffer first")]
    NoLastReplay,

    #[error("Connection timed out after {timeout} seconds. Check OBS is running and WebSocket is enabled")]
    ConnectionTimeout { timeout: u64 },

    #[error("Failed to connect after {attempts} attempts. Verify OBS WebSocket settings and network connectivity")]
    AllConnectionAttemptsFailed { attempts: u32 },

    #[error("Invalid WebSocket URL format: {0}. Expected format: obsws://hostname:port/password")]
    WebSocketUrlParseError(String),
}

/// Result type alias for obs-cmd operations.
///
/// This is a convenience alias for `std::result::Result<T, ObsCmdError>`
/// to simplify error handling throughout the application.
pub type Result<T> = std::result::Result<T, ObsCmdError>;

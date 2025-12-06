use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::result_large_err)]
pub enum ObsCmdError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(#[from] obws::error::Error),

    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),

    #[allow(dead_code)]
    #[error("Command execution failed: {message}")]
    CommandError { message: String },

    #[error("Invalid audio command: {command}")]
    InvalidAudioCommand { command: String },

    #[error("Invalid filter command: {command}")]
    InvalidFilterCommand { command: String },

    #[error("Invalid scene item command: {command}")]
    InvalidSceneItemCommand { command: String },

    #[error("Monitor not available: index {index} out of range")]
    MonitorNotAvailable { index: u32 },

    #[error("No monitor list received from OBS")]
    NoMonitorList,

    #[error("Recording is not active")]
    RecordingNotActive,

    #[error("Recording is paused")]
    RecordingPaused,

    #[error("No last replay found")]
    NoLastReplay,

    #[allow(dead_code)]
    #[error("OBS operation failed: {0}")]
    ObsOperationError(String),

    #[allow(dead_code)]
    #[error("Invalid URL format: {0}")]
    InvalidUrlFormat(String),

    #[error("Connection timeout after {timeout} seconds")]
    ConnectionTimeout { timeout: u64 },

    #[error("All {attempts} connection attempts failed")]
    AllConnectionAttemptsFailed { attempts: u32 },

    #[error("WebSocket URL parsing failed: {0}")]
    WebSocketUrlParseError(String),
}

pub type Result<T> = std::result::Result<T, ObsCmdError>;

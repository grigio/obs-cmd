use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

/// OBS WebSocket connection configuration.
///
/// This struct represents the connection parameters for connecting
/// to an OBS WebSocket server, typically parsed from a URL string.
#[derive(Clone, Debug)]
pub struct ObsWebsocket {
    /// Hostname or IP address of the OBS WebSocket server
    pub hostname: String,
    /// Port number where OBS WebSocket is listening
    pub port: u16,
    /// Optional password for OBS WebSocket authentication
    pub password: Option<String>,
}

impl FromStr for ObsWebsocket {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Url::parse(s) {
            Ok(unvalidated_websocket) => {
                if unvalidated_websocket.scheme() != "obsws" {
                    return Err(
                        "Invalid URL format, use the format obsws://hostname:port/password",
                    );
                }

                let hostname = unvalidated_websocket
                    .host()
                    .ok_or("Invalid hostname in URL")?
                    .to_string();

                let port =
                    match unvalidated_websocket.port() {
                        Some(port) => port,
                        None => return Err(
                            "Please specify a port in the format obsws://hostname:port/password",
                        ),
                    };

                let password = match unvalidated_websocket.path() {
                    "" => None,
                    _ => {
                        let mut pass = unvalidated_websocket.path().to_string();
                        // Otherwise the `/` part of the password in the URL is included.
                        let _ = pass.remove(0);
                        Some(pass)
                    }
                };

                Ok(ObsWebsocket {
                    hostname,
                    port,
                    password,
                })
            }
            Err(_) => Err("Invalid URL format, use the format obsws://hostname:port/password"),
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Replay {
    Start,
    Stop,
    Toggle,
    Save,
    Status,
    LastReplay,
}

#[derive(Subcommand, Clone, Debug)]
pub enum VirtualCamera {
    Start,
    Stop,
    Toggle,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Streaming {
    Start,
    Stop,
    Status,
    Toggle,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Recording {
    Start,
    Stop,
    Toggle,
    Status,
    StatusActive,
    Pause,
    Resume,
    TogglePause,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Scene {
    Current,
    Switch { scene_name: String },
}

#[derive(Subcommand, Clone, Debug)]
pub enum SceneCollection {
    Current,
    Switch { scene_collection_name: String },
}

/// Command-line interface for obs-cmd.
///
/// This struct defines the main CLI interface using clap for parsing.
/// It supports connecting to OBS WebSocket and executing various commands.
///
/// # Examples
///
/// ```bash
/// # Get OBS version info
/// obs-cmd info
///
/// # Start recording
/// obs-cmd recording start
///
/// # Switch to a scene
/// obs-cmd scene switch "Main Scene"
///
/// # Connect to custom OBS WebSocket
/// obs-cmd --websocket obsws://192.168.1.100:4455/password info
/// ```
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// OBS WebSocket connection URL.
    ///
    /// If not provided, defaults to `obsws://localhost:4455/secret`.
    /// Can also be set via OBS_WEBSOCKET_URL environment variable.
    #[clap(short, long)]
    pub websocket: Option<ObsWebsocket>,

    /// The command to execute on OBS.
    #[clap(subcommand)]
    pub command: Commands,
}

/// Available commands for controlling OBS.
///
/// This enum represents all possible operations that can be performed
/// on OBS Studio via the WebSocket interface.
#[derive(Subcommand)]
pub enum Commands {
    /// Get OBS Studio version and information
    Info,
    #[clap(subcommand)]
    Scene(Scene),

    #[clap(subcommand)]
    SceneCollection(SceneCollection),

    #[clap(subcommand)]
    Replay(Replay),

    #[clap(subcommand)]
    VirtualCamera(VirtualCamera),

    #[clap(subcommand)]
    Streaming(Streaming),

    #[clap(subcommand)]
    Recording(Recording),

    SaveScreenshot {
        source: String,
        format: String,
        file_path: PathBuf,
        #[clap(long)]
        width: Option<u32>,
        #[clap(long)]
        height: Option<u32>,
        #[clap(long)]
        compression_quality: Option<i32>,
    },

    Audio {
        command: String,
        device: String,
    },

    Filter {
        command: String,
        source: String,
        filter: String,
    },

    SceneItem {
        command: String,
        scene: String,
        source: String,
    },

    TriggerHotkey {
        name: String,
    },

    FullscreenProjector {
        #[arg(long, default_value_t = 0)]
        monitor_index: u8,
    },

    SourceProjector {
        name: String,
        #[arg(long, default_value_t = 0)]
        monitor_index: u8,
    },
    #[clap(subcommand)]
    MediaInput(MediaInput),
}

#[derive(Subcommand, Clone, Debug)]
pub enum MediaInput {
    /// Sets the cursor of the media input
    SetCursor {
        /// The name of media input
        name: String,
        /// The duration in human readable format for example:
        /// - "00:15" (means 15 seconds)
        /// - "23:15" (means 23 minutes and 15 seconds)
        /// - "1:00:15" (means 1 hour and 15 seconds)
        #[arg(value_parser=parse_duration)]
        cursor: time::Duration,
    },
    /// Starts playing the media input
    Play {
        /// The name of media input
        name: String,
    },
    /// Pauses the media input
    Pause {
        /// The name of media input
        name: String,
    },
    /// Stops the media input
    Stop {
        /// The name of media input
        name: String,
    },
    /// Restarts the media input
    Restart {
        /// The name of media input
        name: String,
    },
}

/// Parses duration strings in [hh:]mm:ss format.
///
/// This function converts human-readable time strings into Duration objects.
/// Supports both minute:second and hour:minute:second formats.
///
/// # Examples
///
/// * "0:00" -> 0 seconds
/// * "01:00" -> 1 minute  
/// * "1:00:00" -> 1 hour
/// * "1:30:45" -> 1 hour, 30 minutes, 45 seconds
///
/// # Arguments
///
/// * `s` - The duration string to parse
///
/// # Returns
///
/// Returns a `time::Duration` on success, or an error string if format is invalid
fn parse_duration(s: &str) -> Result<time::Duration, String> {
    let parts = s
        .split_terminator(':')
        .map(i64::from_str)
        .collect::<Vec<_>>();

    match parts.as_slice() {
        [Ok(m), Ok(s)] => Ok(time::Duration::seconds(m * 60 + s)),
        [Ok(h), Ok(m), Ok(s)] => Ok(time::Duration::seconds(h * 60 * 60 + m * 60 + s)),
        _ => Err("Duration should be of format [hh:]mm:ss".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_duration;

    #[test]
    fn test_parse_duration() {
        assert_eq!(
            parse_duration("0:00").unwrap(),
            time::Duration::milliseconds(0)
        );
        assert_eq!(
            parse_duration("00:00").unwrap(),
            time::Duration::milliseconds(0)
        );
        assert_eq!(
            parse_duration("0:1").unwrap(),
            time::Duration::milliseconds(1000)
        );
        assert_eq!(
            parse_duration("10:12").unwrap(),
            time::Duration::seconds(10 * 60 + 12)
        );
        assert_eq!(
            parse_duration("1:10:12").unwrap(),
            time::Duration::seconds(60 * 60 + 10 * 60 + 12)
        );
    }
}

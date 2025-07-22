use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug)]
pub struct ObsWebsocket {
    pub hostname: String,
    pub port: u16,
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

                let hostname = unvalidated_websocket.host().unwrap().to_string();

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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    /// The default websocket URL is `obsws://localhost:4455/secret`
    /// if this argument is not provided
    pub websocket: Option<ObsWebsocket>,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
        #[arg(value_parser=parse_duration)]
        cursor: time::Duration,
    },
}

// Parses strings of such format:
// 0:00 -> 0 seconds
// 01:00 -> 1 minute
// 1:00:00 -> 1 hour
fn parse_duration(s: &str) -> Result<time::Duration, String> {
    let parts = s.split_terminator(':').collect::<Vec<_>>();
    // "00:00" -> parts => ["00", "00"]
    // "1:00:00" -> parts => ["1", "00", "00"]
    match parts.as_slice() {
        [] | [_] => return Err("Duration should be of format [hh:]mm:ss".into()),
        [m, s] => {
            let m = i64::from_str(m).map_err(|e| format!("Failed to parse minutes: {e}"))?;
            let s = i64::from_str(s).map_err(|e| format!("Failed to parse seconds: {e}"))?;
            return Ok(time::Duration::seconds(m * 60 + s));
        }
        [h, m, s] => {
            let h = i64::from_str(h).map_err(|e| format!("Failed to parse hours: {e}"))?;
            let m = i64::from_str(m).map_err(|e| format!("Failed to parse minutes: {e}"))?;
            let s = i64::from_str(s).map_err(|e| format!("Failed to parse seconds: {e}"))?;
            return Ok(time::Duration::seconds(h * 60 * 60 + m * 60 + s));
        }
        _ => return Err("Duration should be of format [hh:]mm:ss".into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::command::parse_duration;

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

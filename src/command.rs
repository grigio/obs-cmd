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
    Toggle,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Recording {
    Start,
    Stop,
    Toggle,
    Status,
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
    Scene {
        switch_placeholder: String, // NOTE: just for args positioning
        scene_name: String,
    },
    SceneCollection {
        switch_placeholder: String, // NOTE: just for args positioning
        scene_collection_name: String,
    },

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

    ToggleMute {
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
    }

}

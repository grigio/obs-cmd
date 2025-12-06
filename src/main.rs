#![allow(clippy::redundant_closure, clippy::result_large_err)]

mod cli;
mod connection;
mod error;
mod handler;
mod handlers;

use clap::{CommandFactory, Parser};
use cli::{Cli, ObsWebsocket};
use connection::{connect_with_retry, ConnectionConfig};
use error::{ObsCmdError, Result};
use handler::handle_commands;

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle completion command separately since it doesn't need OBS connection
    if let cli::Commands::Completion { shell } = cli.command {
        let mut cmd = Cli::command();
        clap_complete::generate(shell, &mut cmd, "obs-cmd", &mut std::io::stdout());
        return Ok(());
    }

    let config = ConnectionConfig::default();

    let client = match std::env::var("OBS_WEBSOCKET_URL") {
        Ok(url) => {
            let parsed_url = url::Url::parse(&url)?;
            let hostname = parsed_url
                .host_str()
                .ok_or_else(|| {
                    ObsCmdError::WebSocketUrlParseError(
                        "Missing hostname in WebSocket URL".to_string(),
                    )
                })?
                .to_string();
            let port = parsed_url.port().ok_or_else(|| {
                ObsCmdError::WebSocketUrlParseError("Missing port in WebSocket URL".to_string())
            })?;
            let password = parsed_url
                .path_segments()
                .and_then(|mut segments| segments.next())
                .ok_or_else(|| {
                    ObsCmdError::WebSocketUrlParseError(
                        "Missing password in WebSocket URL path".to_string(),
                    )
                })?;

            connect_with_retry(hostname, port, Some(password.to_string()), config).await?
        }
        Err(_) => match cli.websocket {
            Some(ObsWebsocket {
                hostname,
                port,
                password,
            }) => connect_with_retry(hostname, port, password, config).await?,
            None => {
                connect_with_retry(
                    "localhost".to_string(),
                    4455,
                    Some("secret".to_string()),
                    config,
                )
                .await?
            }
        },
    };

    handle_commands(&client, &cli.command).await
}

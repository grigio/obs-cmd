mod cli;
mod handler;

use clap::Parser;
use cli::{Cli, ObsWebsocket};
use handler::handle_commands;
use obws::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = match std::env::var("OBS_WEBSOCKET_URL") {
        Ok(url) => {
            let parsed_url = url::Url::parse(&url).expect("Invalid OBS_WEBSOCKET_URL format");
            let hostname = parsed_url
                .host_str()
                .expect("Hostname not found in OBS_WEBSOCKET_URL")
                .to_string();
            let port = parsed_url
                .port()
                .expect("Port not found in OBS_WEBSOCKET_URL");
            let password = parsed_url
                .path_segments()
                .and_then(|mut segments| segments.next())
                .ok_or(url::ParseError::RelativeUrlWithoutBase)?;

            Client::connect(hostname, port, Some(password)).await?
        }
        Err(_) => match cli.websocket {
            Some(ObsWebsocket {
                hostname,
                port,
                password,
            }) => Client::connect(hostname, port, password).await?,
            None => Client::connect("localhost", 4455, Some("secret")).await?,
        },
    };

    handle_commands(&client, &cli.command).await
}

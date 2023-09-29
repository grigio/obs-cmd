use anyhow::{anyhow, Result};
use obws::{requests::filters::SetEnabled, Client};
use std::fs;
use toml::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./program <command>");
        return Ok(());
    }

    // Determine the configuration directory
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow!("Unable to determine config directory"))?;

    // Read the TOML file
    let config_file_path = config_dir.join("obs-cmd.toml");
    let config_content = fs::read_to_string(&config_file_path)
        .map_err(|_| anyhow!("Unable to read the TOML file: {:?}", &config_file_path))?;

    // Parse the TOML content
    let config: Value =
        toml::from_str(&config_content).map_err(|_| anyhow!("Unable to parse the TOML content"))?;

    // Extract the value of OBS_WS_PASSWORD
    let obs_ws_password = config
        .get("OBS_WS_PASSWORD")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("OBS_WS_PASSWORD not found in the TOML file"))?;

    // TODO: URL like ws://pass@localhost:4455?
    let client = Client::connect("localhost", 4455, Some(obs_ws_password)).await?;

    match args[1].as_str() {
        "info" => {
            // CLI info
            let version = client.general().version().await?;
            println!("Version: {:?}", version);
        }
        "scene" => {
            let scene_name = &args[3];
            let res = client.scenes().set_current_program_scene(scene_name).await;
            println!("Set current scene: {}", scene_name);
            println!("Result: {:?}", res);
        }
        "recording" => {
            if args.len() < 3 {
                println!("Usage: ./program recording <command>");
                return Ok(());
            }
            let command = &args[2];
            match command.as_str() {
                "start" => {
                    let res = client.recording().start().await;
                    println!("Recording started");
                    println!("Result: {:?}", res);
                }
                "stop" => {
                    let res = client.recording().stop().await;
                    println!("Recording stopped");
                    println!("Result: {:?}", res);
                }
                "toggle" => {
                    let res = client.recording().toggle().await;
                    println!("Recording toggled");
                    println!("Result: {:?}", res);
                }
                _ => {
                    println!("Invalid recording command: {}", command);
                }
            }
        }
        "replay" => {
            if args.len() < 3 {
                println!("Usage: ./program replay <command>");
                return Ok(());
            }
            let command = &args[2];
            match command.as_str() {
                "start" => {
                    let res = client.replay_buffer().start().await;
                    println!("Replay Buffer started");
                    println!("Result: {:?}", res);
                }
                "stop" => {
                    let res = client.replay_buffer().stop().await;
                    println!("Replay Buffer stopped");
                    println!("Result: {:?}", res);
                }
                "toggle" => {
                    let res = client.replay_buffer().toggle().await?;
                    println!("Replay Buffer toggled");
                    println!("Result: {:?}", res);
                }
                "save" => {
                    let res = client.replay_buffer().save().await;
                    println!("Buffer saved");
                    println!("Result: {:?}", res);
                }
                _ => {
                    println!("Invalid replay command: {}", command);
                }
            }
        }
        "streaming" => {
            if args.len() < 3 {
                println!("Usage: ./program streaming <command>");
                return Ok(());
            }
            let command = &args[2];
            match command.as_str() {
                "start" => {
                    let res = client.streaming().start().await;
                    println!("Streaming started");
                    println!("Result: {:?}", res);
                }
                "stop" => {
                    let res = client.streaming().stop().await;
                    println!("Streaming stopped");
                    println!("Result: {:?}", res);
                }
                "toggle" => {
                    let res = client.streaming().toggle().await?;
                    println!("Streaming toggled");
                    println!("Result: {:?}", res);
                }
                _ => {
                    println!("Invalid streaming command: {}", command);
                }
            }
        }
        "virtualcam" => {
            if args.len() < 3 {
                println!("Usage: ./program virtualcam <command>");
                return Ok(());
            }
            let command = &args[2];
            match command.as_str() {
                "start" => {
                    let res = client.virtual_cam().start().await;
                    println!("Result: {:?}", res);
                }
                "stop" => {
                    let res = client.virtual_cam().stop().await;
                    println!("Result: {:?}", res);
                }
                "toggle" => {
                    let res = client.virtual_cam().toggle().await?;
                    println!("Result: {:?}", res);
                }
                _ => {
                    println!("Invalid virtualcam command: {}", command);
                }
            }
        }
        "filter" => {
            if args.len() < 5 {
                println!("Usage: ./program filter <command> <source-name> <filter-name>");
                return Ok(());
            }
            let command = &args[2];
            let source = &args[3];
            let filter = &args[4];
            let enabled: bool = match command.as_str() {
                "enable" => true,
                "disable" => false,
                "toggle" => !client.filters().get(source, filter).await?.enabled,
                _ => {
                    println!("Invalid filter command: {}", command);
                    return Ok(());
                }
            };
            let res = client
                .filters()
                .set_enabled(SetEnabled {
                    source,
                    filter,
                    enabled,
                })
                .await;
            println!("Result: {:?}", res);
        }
        _ => {
            println!("Invalid command: {}", args[1]);
        }
    }

    Ok(())
}

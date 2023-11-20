use clap::{Parser, Subcommand};
use obws::{requests::filters::SetEnabled, Client};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    obsws: Option<String>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Info,
    Scene {
        switch_placeholder: String, // NOTE: just for args positioning
        scene_name: String,
    },
    Replay {
        action: String,
    },
    Virtualcam {
        action: String,
    },
    Streaming {
        action: String,
    },
    Recording {
        action: String,
    },
    ToggleMute {
        device: String,
    },
    Filter {
        command: String,
        source: String,
        filter: String,
    },
}

fn parse_obsws(input: &str) -> Result<(&str, &str, u16), &'static str> {
    if !input.starts_with("obsws://") {
        return Err("Invalid URL format, use the format: obsws://hostname:port/password");
    }

    let without_prefix = &input[8..];
    let parts: Vec<&str> = without_prefix.split([':', '/'].as_ref()).collect();

    if parts.len() < 3 {
        return Err("Invalid format");
    }

    let hostname = parts[0];
    let port = parts[1].parse().map_err(|_| "Invalid port number")?;
    let password = parts[2];

    Ok((hostname, password, port))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    let obs_ws_url = cli
        .obsws
        .unwrap_or_else(|| String::from("obsws://localhost:4455/secret"));

    let (hostname, password, port) = parse_obsws(&obs_ws_url)?;
    let client = Client::connect(hostname, port, Some(password)).await?;

    match &cli.command {
        Commands::Scene {
            switch_placeholder,
            scene_name,
        } => {
            // let scene_name = &args[3];
            let res = client.scenes().set_current_program_scene(scene_name).await;
            println!("Set current scene: {} {}", switch_placeholder, scene_name);
            println!("Result: {:?}", res);
        }
        Commands::Info => {
            let version = client.general().version().await?;
            println!("Version: {:?}", version);
        }
        Commands::Recording { action } => {
            println!("Recording {:?}", action);
            match action.as_str() {
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
                    println!("Invalid recording command: {}", action);
                }
            }
        }
        Commands::Streaming { action } => {
            println!("Streaming {:?}", action);
            match action.as_str() {
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
                    println!("Invalid streaming command: {}", action);
                }
            }
        }
        Commands::Virtualcam { action } => {
            println!("Virtualcam {:?}", action);
            match action.as_str() {
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
                    println!("Invalid virtualcam command: {}", action);
                }
            }
        }
        Commands::Replay { action } => {
            println!("Replay {:?}", action);
            match action.as_str() {
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
                    println!("Invalid replay command: {}", action);
                }
            }
        }
        Commands::ToggleMute { device } => {
            println!("Toggle mute device: {:?}  ", device);

            let res = client.inputs().toggle_mute(device).await;
            println!("Result: {:?}", res);
        }
        Commands::Filter {
            command,
            source,
            filter,
        } => {
            println!("Filter: {:?} {:?} {:?}", command, source, filter);

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
    }

    Ok(())
}

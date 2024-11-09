mod command;
use command::*;

use clap::Parser;
use obws::{requests::filters::SetEnabled as SetEnabledFilter, Client};
use obws::requests::scene_items::SetEnabled as SetEnabledItem;
use obws::requests::scene_items::Id as IdItem;
use obws::requests::sources::SaveScreenshot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = match cli.websocket {
        Some(ObsWebsocket {
            hostname,
            port,
            password,
        }) => Client::connect(hostname, port, password).await?,
        None => Client::connect("localhost", 4455, Some("secret")).await?,
    };

    match &cli.command {
        Commands::Scene(action) => {
            use Scene::*;

            match action {
                Current => {
                    let scene_name = client.scenes().current_program_scene().await.and_then(|r| Ok(r))?;
                    println!("{}", scene_name);
                },
                Switch{scene_name} => {
                    let res = client.scenes().set_current_program_scene(scene_name).await;
                    println!("Set current scene: switch {}", scene_name);
                    println!("Result: {:?}", res);
                },
            }
        }

        Commands::SceneCollection {
            switch_placeholder,
            scene_collection_name,
        } => {
            // let scene_name = &args[3];
            let res = client.scene_collections().set_current(scene_collection_name).await;
            println!("Set current scene collection: {} {}", switch_placeholder, scene_collection_name);
            println!("Result: {:?}", res);
        }

        Commands::Info => {
            let version = client.general().version().await?;
            println!("Version: {:?}", version);
        }

        Commands::Recording(action) => {
            use Recording::*;
            println!("Recording {:?}", action);

            match action {
                Start => {
                    let res = client.recording().start().await;
                    println!("Recording started");
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.recording().stop().await;
                    println!("Recording stopped");
                    println!("Result: {:?}", res);
                }
                Toggle => {
                    let res = client.recording().toggle().await;
                    println!("Recording toggled");
                    println!("Result: {:?}", res);
                }
                Status => {
                    let status = client.recording().status().await?;
                    println!("Recording: {:?}", status.active);
                    if status.active {
                        println!("Paused: {:?}", status.paused);
                        println!("Timecode: {:?}", status.timecode);
                        println!("Bytes: {:?}", status.bytes);
                    }
                }
                Pause => {
                    let res = client.recording().pause().await;
                    println!("Recording paused");
                    println!("Result: {:?}", res);
                }
                Resume => {
                    let res = client.recording().resume().await;
                    println!("Recording resumed");
                    println!("Result: {:?}", res);
                }
                TogglePause => {
                    let res = client.recording().toggle_pause().await;
                    println!("Recording pause toggled");
                    println!("Result: {:?}", res);
                }
            }
        }

        Commands::SaveScreenshot {
            source,
            format,
            width,
            height,
            compression_quality,
            file_path,
        } => {
            let settings = SaveScreenshot {
                source,
                format,
                width: *width,
                height: *height,
                compression_quality: *compression_quality,
                file_path,
            };
            client.sources().save_screenshot(settings).await?;
            println!("Saved screenshot to path: {:?}", file_path);
        }

        Commands::Streaming(action) => {
            use Streaming::*;
            println!("Streaming {:?}", action);

            match action {
                Start => {
                    let res = client.streaming().start().await;
                    println!("Streaming started");
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.streaming().stop().await;
                    println!("Streaming stopped");
                    println!("Result: {:?}", res);
                }
                Status => {
                    let res = client.streaming().status().await?;
                    println!("Streaming: {:?}", res.active);
                }
                Toggle => {
                    let res = client.streaming().toggle().await?;
                    println!("Streaming toggled");
                    println!("Result: {:?}", res);
                }
            }
        }

        Commands::VirtualCamera(action) => {
            use VirtualCamera::*;
            println!("VirtualCamera {:?}", action);

            match action {
                Start => {
                    let res = client.virtual_cam().start().await;
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.virtual_cam().stop().await;
                    println!("Result: {:?}", res);
                }
                Toggle => {
                    let res = client.virtual_cam().toggle().await?;
                    println!("Result: {:?}", res);
                }
            }
        }

        Commands::Replay(action) => {
            use Replay::*;
            println!("Replay {:?}", action);

            match action {
                Start => {
                    let res = client.replay_buffer().start().await;
                    println!("Replay Buffer started");
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.replay_buffer().stop().await;
                    println!("Replay Buffer stopped");
                    println!("Result: {:?}", res);
                }
                Toggle => {
                    let res = client.replay_buffer().toggle().await?;
                    println!("Replay Buffer toggled");
                    println!("Result: {:?}", res);
                }
                Save => {
                    let res = client.replay_buffer().save().await;
                    println!("Buffer saved");
                    println!("Result: {:?}", res);
                }
                Status => {
                    let res = client.replay_buffer().status().await?;
                    println!(
                        "Replay Buffer is {}",
                        if res { "running" } else { "not running" }
                    );
                }
                LastReplay => {
                    let res = client.replay_buffer().last_replay().await?;
                    if res.is_empty() {
                        println!("No last replay found");
                    } else {
                        println!("Replay path: {}", res);
                    }
                }
            }
        }

        Commands::ToggleMute { device } => {
            println!("Toggling mute on device: {:?}  ", device);

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
                .set_enabled(SetEnabledFilter {
                    source,
                    filter,
                    enabled,
                })
                .await;
            println!("Result: {:?}", res);
        }

        Commands::SceneItem {
            command,
            scene,
            source,
        } => {
            println!("Scene Item: {:?} {:?} {:?}", command, scene, source);

            // get item_id
            let item_id = client
                          .scene_items()
                          .id(IdItem {
                              scene,
                              source,
                              search_offset: Some(0)
                          })
                          .await?;

            // use item_id in toggle
            let enabled: bool = match command.as_str() {
                "enable" => true,
                "disable" => false,
                "toggle" => !client.scene_items().enabled(scene, item_id).await?,
                _ => {
                    println!("Invalid scene item command: {}", command);
                    return Ok(());
                }
            }; // use item_id in setenabled
            let res = client
                .scene_items()
                .set_enabled(SetEnabledItem {
                    scene,
                    item_id,
                    enabled,
                })
                .await;
            println!("Result: {:?}", res);
        }

    }

    Ok(())
}

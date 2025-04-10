mod command;
use command::*;

use clap::Parser;
use obws::{requests::filters::SetEnabled as SetEnabledFilter, Client};
use obws::{requests::scene_items::SetEnabled as SetEnabledItem};
use obws::{requests::scene_items::Id as IdItem};
use obws::{requests::sources::SaveScreenshot};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

let client = match std::env::var("OBS_WEBSOCKET_URL") {
    Ok(url) => {
        let parsed_url = url::Url::parse(&url).expect("Invalid OBS_WEBSOCKET_URL format");
        let hostname = parsed_url.host_str().expect("Hostname not found in OBS_WEBSOCKET_URL").to_string();
        let port = parsed_url.port().expect("Port not found in OBS_WEBSOCKET_URL");
        let password = parsed_url.path_segments().and_then(|mut segments| segments.next()).ok_or(url::ParseError::RelativeUrlWithoutBase)?;

        // let password = parsed_url.password().map(|p| p.to_string());
        Client::connect(hostname, port, Some(password)).await?
    },
    Err(_) => match cli.websocket {
        Some(ObsWebsocket {
            hostname,
            port,
            password,
        }) => Client::connect(hostname, port, password).await?,
        None => Client::connect("localhost", 4455, Some("secret")).await?,
    },
};

    match &cli.command {
        Commands::Scene(action) => {
            use Scene::*;

            match action {
                Current => {
                    let scene_name = client.scenes().current_program_scene().await?;
                    println!("{:?}", scene_name);
                },
                Switch{scene_name} => {
                    let res = client.scenes().set_current_program_scene(scene_name).await;
                    println!("Set current scene: switch {:?}", scene_name);
                    println!("Result: {:?}", res);
                    res?;
                },
            }
        }

        Commands::SceneCollection(action) => {
            use SceneCollection::*;

            match action {
                Current => {
                    let scene_collection_name = client.scene_collections().current().await?;
                    println!("{:?}", scene_collection_name);
                },
                Switch{scene_collection_name} => {
                    let res = client.scene_collections().set_current(scene_collection_name).await;
                    println!("Set current scene collection: {:?}", scene_collection_name);
                    println!("Result: {:?}", res);
                    res?;
                },
            }
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
                    println!("Start recording");
                    println!("Result: {:?}", res);
                    res?;
                }
                Stop => {
                    let res = client.recording().stop().await;
                    println!("Stop recording");
                    println!("Result: {:?}", res);
                    res?;
                }
                Toggle => {
                    let res = client.recording().toggle().await;
                    println!("Toggle recording");
                    println!("Result: {:?}", res);
                    res?;
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
                StatusActive => {
                    let status = client.recording().status().await?;
                    if status.active && !status.paused {
                        println!("Active (started and running)");
                    } else if !status.active {
                        let error_message = "Inactive (not started)";
                        println!("{error_message}");
                        Err(error_message)?;
                    } else {
                        let error_message = "Inactive (not running)";
                        println!("{error_message}");
                        Err(error_message)?;
                    }
                }
                Pause => {
                    let res = client.recording().pause().await;
                    println!("Pause recording");
                    println!("Result: {:?}", res);
                    res?;
                }
                Resume => {
                    let res = client.recording().resume().await;
                    println!("Resume recording");
                    println!("Result: {:?}", res);
                    res?;
                }
                TogglePause => {
                    let res = client.recording().toggle_pause().await;
                    println!("Toggle recording pause");
                    println!("Result: {:?}", res);
                    res?;
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
                    println!("Start streaming");
                    println!("Result: {:?}", res);
                    res?;
                }
                Stop => {
                    let res = client.streaming().stop().await;
                    println!("Stop streaming");
                    println!("Result: {:?}", res);
                    res?;
                }
                Status => {
                    let res = client.streaming().status().await?;
                    println!("Streaming status: {:?}", res.active);
                }
                Toggle => {
                    let res = client.streaming().toggle().await?;
                    println!("Toggle streaming");
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
                    res?;
                }
                Stop => {
                    let res = client.virtual_cam().stop().await;
                    println!("Result: {:?}", res);
                    res?;
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
                    println!("Start Replay Buffer");
                    println!("Result: {:?}", res);
                    res?;
                }
                Stop => {
                    let res = client.replay_buffer().stop().await;
                    println!("Stop Replay Buffer");
                    println!("Result: {:?}", res);
                    res?;
                }
                Toggle => {
                    let res = client.replay_buffer().toggle().await?;
                    println!("Toggle Replay Buffer");
                    println!("Result: {:?}", res);
                }
                Save => {
                    println!("Save buffer");
                    let res = client.replay_buffer().save().await;
                    println!("Result: {:?}", res);
                    res?;
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
                        let error_message = "No last replay found";
                        println!("{error_message}");
                        Err(error_message)?;
                    } 
                    println!("Replay path: {:?}", res);
                }
            }
        }

        Commands::Audio {
            command,
            device
        } => {
            println!("Audio: {:?} {:?}", command, device);

            let muted: bool = match command.as_str() {
                "mute" => true,
                "unmute" => false,
                "toggle" => !client.inputs().muted(device).await?,
                "status" => {
                    let status = client.inputs().muted(device).await?;
                    println!("Muted: {:?}", status);
                    return Ok(());
                }
                _ => {
                    let error_message = format!("Invalid audio command: {:?}", command);
                    println!("{error_message}");
                    return Err(error_message)?;
                }
            };
            let res = client.inputs().set_muted(device, muted).await;
            println!("Result: {:?}", res);
            res?;
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
                    let error_message = format!("Invalid filter command: {:?}", command);
                    println!("{error_message}");
                    return Err(error_message)?;
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
            res?;
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
                    let error_message = format!("Invalid scene item command: {:?}", command);
                    println!("{error_message}");
                    return Err(error_message)?;
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
            res?;
        }

        Commands::TriggerHotkey {
            name
        } => {
            println!("Trigger Hotkey: {:?}", name);
            let res = client.hotkeys().trigger_by_name(name).await;
            println!("Result: {:?}", res);
            res?;
        }

        Commands::FullscreenProjector {
        } => {
            use obws::{requests::ui::OpenVideoMixProjector};
            use obws::{requests::ui::VideoMixType::Program as OpenVideoMixProjectorType};
            use obws::{requests::ui::Location::MonitorIndex as MonitorLocationIndex};
            println!("Open fullscreen projector");
            let monitor_list_res = client.ui().list_monitors().await;
            if monitor_list_res.is_ok() {
                let monitor_list = monitor_list_res.unwrap();
                if monitor_list.len() > 0 {
                    let res = client.ui().open_video_mix_projector(OpenVideoMixProjector{ r#type: OpenVideoMixProjectorType, location:
                        Some(MonitorLocationIndex(monitor_list[0].index as i32))
                    }).await;
                    println!("Result: {:?}", res);
                    res?;
                } else {
                    Err("No monitor in list")?;
                }
            } else {
                Err("No monitor list received")?;
            }
        }

    }

    Ok(())
}

use crate::cli::{
    Commands, MediaInput, Recording, Replay, Scene, SceneCollection, Streaming, VirtualCamera,
};
use crate::connection::check_connection_health;
use crate::error::{ObsCmdError, Result};
use obws::common::MediaAction;
use obws::requests::filters::SetEnabled as SetEnabledFilter;
use obws::requests::scene_items::Id as IdItem;
use obws::requests::scene_items::SetEnabled as SetEnabledItem;
use obws::requests::sources::SaveScreenshot;
use obws::Client;

pub async fn handle_commands(client: &Client, commands: &Commands) -> Result<()> {
    // Check connection health before executing commands
    if let Err(e) = check_connection_health(client).await {
        eprintln!("Warning: Connection health check failed: {}", e);
        // Continue with command execution but warn user
    }
    match commands {
        Commands::MediaInput(media_input) => match media_input {
            MediaInput::SetCursor { name, cursor } => {
                client
                    .media_inputs()
                    .set_cursor(name.as_str().into(), *cursor)
                    .await?;
                println!("Media input {name}'s cursor was set to: {cursor:?}");
            }
            MediaInput::Play { name } => {
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Play)
                    .await?;
                println!("Media input {name} is playing");
            }
            MediaInput::Restart { name } => {
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Restart)
                    .await?;
                println!("Media input {name} is restarted");
            }
            MediaInput::Pause { name } => {
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Pause)
                    .await?;
                println!("Media input {name} is paused");
            }
            MediaInput::Stop { name } => {
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Stop)
                    .await?;
                println!("Media input {name} is stopped");
            }
        },
        Commands::Scene(action) => {
            use Scene::*;

            match action {
                Current => {
                    let scene_name = client.scenes().current_program_scene().await?;
                    println!("{scene_name:?}");
                }
                Switch { scene_name } => {
                    let res = client
                        .scenes()
                        .set_current_program_scene(scene_name.as_str())
                        .await;
                    println!("Set current scene: switch {scene_name:?}");
                    println!("Result: {res:?}");
                    res?;
                }
            }
        }

        Commands::SceneCollection(action) => {
            use SceneCollection::*;

            match action {
                Current => {
                    let scene_collection_name = client.scene_collections().current().await?;
                    println!("{scene_collection_name:?}");
                }
                Switch {
                    scene_collection_name,
                } => {
                    let res = client
                        .scene_collections()
                        .set_current(scene_collection_name)
                        .await;
                    println!("Set current scene collection: {scene_collection_name:?}");
                    println!("Result: {res:?}");
                    res?;
                }
            }
        }

        Commands::Info => {
            let version = client.general().version().await?;
            println!("Version: {version:?}");
        }

        Commands::Recording(action) => {
            use Recording::*;
            println!("Recording {action:?}");

            match action {
                Start => {
                    let res = client.recording().start().await;
                    println!("Start recording");
                    println!("Result: {res:?}");
                    res?;
                }
                Stop => {
                    let res = client.recording().stop().await;
                    println!("Stop recording");
                    println!("Result: {res:?}");
                    res?;
                }
                Toggle => {
                    let res = client.recording().toggle().await;
                    println!("Toggle recording");
                    println!("Result: {res:?}");
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
                        Err(ObsCmdError::RecordingNotActive)?;
                    } else {
                        Err(ObsCmdError::RecordingPaused)?;
                    }
                }
                Pause => {
                    let res = client.recording().pause().await;
                    println!("Pause recording");
                    println!("Result: {res:?}");
                    res?;
                }
                Resume => {
                    let res = client.recording().resume().await;
                    println!("Resume recording");
                    println!("Result: {res:?}");
                    res?;
                }
                TogglePause => {
                    let res = client.recording().toggle_pause().await;
                    println!("Toggle recording pause");
                    println!("Result: {res:?}");
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
                source: obws::requests::sources::SourceId::Name(source.as_str()),
                format,
                width: *width,
                height: *height,
                compression_quality: *compression_quality,
                file_path,
            };
            client.sources().save_screenshot(settings).await?;
            println!("Saved screenshot to path: {file_path:?}");
        }

        Commands::Streaming(action) => {
            use Streaming::*;
            println!("Streaming {action:?}");

            match action {
                Start => {
                    let res = client.streaming().start().await;
                    println!("Start streaming");
                    println!("Result: {res:?}");
                    res?;
                }
                Stop => {
                    let res = client.streaming().stop().await;
                    println!("Stop streaming");
                    println!("Result: {res:?}");
                    res?;
                }
                Status => {
                    let res = client.streaming().status().await?;
                    println!("Streaming status: {:?}", res.active);
                }
                Toggle => {
                    let res = client.streaming().toggle().await?;
                    println!("Toggle streaming");
                    println!("Result: {res:?}");
                }
            }
        }

        Commands::VirtualCamera(action) => {
            use VirtualCamera::*;
            println!("VirtualCamera {action:?}");

            match action {
                Start => {
                    let res = client.virtual_cam().start().await;
                    println!("Result: {res:?}");
                    res?;
                }
                Stop => {
                    let res = client.virtual_cam().stop().await;
                    println!("Result: {res:?}");
                    res?;
                }
                Toggle => {
                    let res = client.virtual_cam().toggle().await?;
                    println!("Result: {res:?}");
                }
            }
        }

        Commands::Replay(action) => {
            use Replay::*;
            println!("Replay {action:?}");

            match action {
                Start => {
                    let res = client.replay_buffer().start().await;
                    println!("Start Replay Buffer");
                    println!("Result: {res:?}");
                    res?;
                }
                Stop => {
                    let res = client.replay_buffer().stop().await;
                    println!("Stop Replay Buffer");
                    println!("Result: {res:?}");
                    res?;
                }
                Toggle => {
                    let res = client.replay_buffer().toggle().await?;
                    println!("Toggle Replay Buffer");
                    println!("Result: {res:?}");
                }
                Save => {
                    println!("Save buffer");
                    let res = client.replay_buffer().save().await;
                    println!("Result: {res:?}");
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
                        return Err(ObsCmdError::NoLastReplay);
                    }
                    println!("Replay path: {res:?}");
                }
            }
        }

        Commands::Audio { command, device } => {
            println!("Audio: {command:?} {device:?}");

            let muted: bool = match command.as_str() {
                "mute" => true,
                "unmute" => false,
                "toggle" => !client.inputs().muted(device.as_str().into()).await?,
                "status" => {
                    let status = client.inputs().muted(device.as_str().into()).await?;
                    println!("Muted: {status:?}");
                    return Ok(());
                }
                _ => {
                    return Err(ObsCmdError::InvalidAudioCommand {
                        command: command.clone(),
                    });
                }
            };
            let res = client
                .inputs()
                .set_muted(device.as_str().into(), muted)
                .await;
            println!("Result: {res:?}");
            res?;
        }

        Commands::Filter {
            command,
            source,
            filter,
        } => {
            println!("Filter: {command:?} {source:?} {filter:?}");

            let enabled: bool = match command.as_str() {
                "enable" => true,
                "disable" => false,
                "toggle" => {
                    !client
                        .filters()
                        .get(source.as_str().into(), filter)
                        .await?
                        .enabled
                }
                _ => {
                    return Err(ObsCmdError::InvalidFilterCommand {
                        command: command.clone(),
                    });
                }
            };
            let res = client
                .filters()
                .set_enabled(SetEnabledFilter {
                    source: source.as_str().into(),
                    filter,
                    enabled,
                })
                .await;
            println!("Result: {res:?}");
            res?;
        }

        Commands::SceneItem {
            command,
            scene,
            source,
        } => {
            println!("Scene Item: {command:?} {scene:?} {source:?}");

            // get item_id
            let item_id = client
                .scene_items()
                .id(IdItem {
                    scene: scene.as_str().into(),
                    source: source.as_str(),
                    search_offset: Some(0),
                })
                .await?;

            // use item_id in toggle
            let enabled: bool = match command.as_str() {
                "enable" => true,
                "disable" => false,
                "toggle" => {
                    !client
                        .scene_items()
                        .enabled(scene.as_str().into(), item_id)
                        .await?
                }
                _ => {
                    return Err(ObsCmdError::InvalidSceneItemCommand {
                        command: command.clone(),
                    });
                }
            }; // use item_id in setenabled
            let res = client
                .scene_items()
                .set_enabled(SetEnabledItem {
                    scene: scene.as_str().into(),
                    item_id,
                    enabled,
                })
                .await;
            println!("Result: {res:?}");
            res?;
        }

        Commands::TriggerHotkey { name } => {
            println!("Trigger Hotkey: {name:?}");
            let res = client.hotkeys().trigger_by_name(name, None).await;
            println!("Result: {res:?}");
            res?;
        }

        Commands::FullscreenProjector { monitor_index } => {
            use obws::requests::ui::Location::MonitorIndex as MonitorLocationIndex;
            use obws::requests::ui::OpenVideoMixProjector;
            use obws::requests::ui::VideoMixType::Program as OpenVideoMixProjectorType;
            println!("Open fullscreen projector");
            if let Ok(monitor_list) = client.ui().list_monitors().await {
                if monitor_list.len() > (*monitor_index as usize) {
                    let res = client
                        .ui()
                        .open_video_mix_projector(OpenVideoMixProjector {
                            r#type: OpenVideoMixProjectorType,
                            location: Some(MonitorLocationIndex(*monitor_index as i32)),
                        })
                        .await;
                    println!("Result: {res:?}");
                    res?;
                } else {
                    return Err(ObsCmdError::MonitorNotAvailable {
                        index: *monitor_index as u32,
                    });
                }
            } else {
                return Err(ObsCmdError::NoMonitorList);
            }
        }

        Commands::SourceProjector {
            name,
            monitor_index,
        } => {
            use obws::requests::ui::Location::MonitorIndex as MonitorLocationIndex;
            use obws::requests::ui::OpenSourceProjector;
            println!("Open source projector");
            if let Ok(monitor_list) = client.ui().list_monitors().await {
                if monitor_list.len() > (*monitor_index as usize) {
                    let res = client
                        .ui()
                        .open_source_projector(OpenSourceProjector {
                            source: name.as_str().into(),
                            location: Some(MonitorLocationIndex(*monitor_index as i32)),
                        })
                        .await;
                    println!("Result: {res:?}");
                    res?;
                } else {
                    return Err(ObsCmdError::MonitorNotAvailable {
                        index: *monitor_index as u32,
                    });
                }
            } else {
                return Err(ObsCmdError::NoMonitorList);
            }
        }
    }
    Ok(())
}

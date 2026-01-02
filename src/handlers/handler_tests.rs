#[cfg(test)]
mod tests {
    use crate::cli::{
        Commands, Input, MediaInput, MuteAction, Recording, Replay, Scene, SceneCollection,
        SceneItem, Streaming, VirtualCamera,
    };
    use crate::handlers::{
        audio::AudioHandler, filters::FilterHandler, general::HotkeyHandler, general::InfoHandler,
        media::MediaInputHandler, recording::RecordingHandler,
        scene_collections::SceneCollectionHandler, scenes::SceneHandler, sources::SourceHandler,
        streaming::StreamingHandler, ui::FullscreenProjectorHandler, ui::SourceProjectorHandler,
        validate_monitor_index, virtual_camera::VirtualCameraHandler, CommandHandler,
    };
    use std::path::PathBuf;

    // Mock client for testing (placeholder for future implementation)
    // struct MockClient {
    //     // In a real implementation, this would contain mock data
    // }

    // impl MockClient {
    //     fn new() -> Self {
    //         Self {}
    //     }
    // }

    // Helper function to create a test client (placeholder for future implementation)
    // async fn create_test_client() -> Client {
    //     // This would create a mock client in a real implementation
    //     // For now, we'll skip actual client creation in tests
    //     panic!("Mock client implementation needed");
    // }

    // Command parsing tests (merged from src/handler_tests.rs)
    #[tokio::test]
    async fn test_media_input_play_command() {
        let command = Commands::MediaInput(MediaInput::Play {
            name: "test_media".to_string(),
        });

        match command {
            Commands::MediaInput(MediaInput::Play { name }) => {
                assert_eq!(name, "test_media");
            }
            _ => panic!("Expected MediaInput::Play command"),
        }
    }

    #[tokio::test]
    async fn test_media_input_pause_command() {
        let command = Commands::MediaInput(MediaInput::Pause {
            name: "test_media".to_string(),
        });

        match command {
            Commands::MediaInput(MediaInput::Pause { name }) => {
                assert_eq!(name, "test_media");
            }
            _ => panic!("Expected MediaInput::Pause command"),
        }
    }

    #[tokio::test]
    async fn test_media_input_stop_command() {
        let command = Commands::MediaInput(MediaInput::Stop {
            name: "test_media".to_string(),
        });

        match command {
            Commands::MediaInput(MediaInput::Stop { name }) => {
                assert_eq!(name, "test_media");
            }
            _ => panic!("Expected MediaInput::Stop command"),
        }
    }

    #[tokio::test]
    async fn test_media_input_restart_command() {
        let command = Commands::MediaInput(MediaInput::Restart {
            name: "test_media".to_string(),
        });

        match command {
            Commands::MediaInput(MediaInput::Restart { name }) => {
                assert_eq!(name, "test_media");
            }
            _ => panic!("Expected MediaInput::Restart command"),
        }
    }

    #[tokio::test]
    async fn test_scene_current_command() {
        let command = Commands::Scene(Scene::Current);

        match command {
            Commands::Scene(Scene::Current) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::Current command"),
        }
    }

    #[tokio::test]
    async fn test_scene_switch_command() {
        let command = Commands::Scene(Scene::Switch {
            scene_name: "test_scene".to_string(),
        });

        match command {
            Commands::Scene(Scene::Switch { scene_name }) => {
                assert_eq!(scene_name, "test_scene");
            }
            _ => panic!("Expected Scene::Switch command"),
        }
    }

    #[tokio::test]
    async fn test_scene_list_command() {
        let command = Commands::Scene(Scene::List);

        match command {
            Commands::Scene(Scene::List) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::List command"),
        }
    }

    #[tokio::test]
    async fn test_scene_create_command() {
        let command = Commands::Scene(Scene::Create {
            scene_name: "new_scene".to_string(),
        });

        match command {
            Commands::Scene(Scene::Create { scene_name }) => {
                assert_eq!(scene_name, "new_scene");
            }
            _ => panic!("Expected Scene::Create command"),
        }
    }

    #[tokio::test]
    async fn test_scene_remove_command() {
        let command = Commands::Scene(Scene::Remove {
            scene_name: "old_scene".to_string(),
        });

        match command {
            Commands::Scene(Scene::Remove { scene_name }) => {
                assert_eq!(scene_name, "old_scene");
            }
            _ => panic!("Expected Scene::Remove command"),
        }
    }

    #[tokio::test]
    async fn test_scene_rename_command() {
        let command = Commands::Scene(Scene::Rename {
            scene_name: "old_name".to_string(),
            new_name: "new_name".to_string(),
        });

        match command {
            Commands::Scene(Scene::Rename {
                scene_name,
                new_name,
            }) => {
                assert_eq!(scene_name, "old_name");
                assert_eq!(new_name, "new_name");
            }
            _ => panic!("Expected Scene::Rename command"),
        }
    }

    #[tokio::test]
    async fn test_scene_transition_list_command() {
        let command = Commands::Scene(Scene::TransitionList);

        match command {
            Commands::Scene(Scene::TransitionList) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::TransitionList command"),
        }
    }

    #[tokio::test]
    async fn test_scene_transition_current_command() {
        let command = Commands::Scene(Scene::TransitionCurrent);

        match command {
            Commands::Scene(Scene::TransitionCurrent) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::TransitionCurrent command"),
        }
    }

    #[tokio::test]
    async fn test_scene_transition_set_command() {
        let command = Commands::Scene(Scene::TransitionSet {
            transition_name: "Fade".to_string(),
        });

        match command {
            Commands::Scene(Scene::TransitionSet { transition_name }) => {
                assert_eq!(transition_name, "Fade");
            }
            _ => panic!("Expected Scene::TransitionSet command"),
        }
    }

    #[tokio::test]
    async fn test_scene_transition_duration_command() {
        let command = Commands::Scene(Scene::TransitionDuration { duration_ms: 500 });

        match command {
            Commands::Scene(Scene::TransitionDuration { duration_ms }) => {
                assert_eq!(duration_ms, 500);
            }
            _ => panic!("Expected Scene::TransitionDuration command"),
        }
    }

    #[tokio::test]
    async fn test_scene_transition_trigger_command() {
        let command = Commands::Scene(Scene::TransitionTrigger);

        match command {
            Commands::Scene(Scene::TransitionTrigger) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::TransitionTrigger command"),
        }
    }

    #[tokio::test]
    async fn test_scene_studio_mode_status_command() {
        let command = Commands::Scene(Scene::StudioModeStatus);

        match command {
            Commands::Scene(Scene::StudioModeStatus) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::StudioModeStatus command"),
        }
    }

    #[tokio::test]
    async fn test_scene_studio_mode_enable_command() {
        let command = Commands::Scene(Scene::StudioModeEnable);

        match command {
            Commands::Scene(Scene::StudioModeEnable) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::StudioModeEnable command"),
        }
    }

    #[tokio::test]
    async fn test_scene_studio_mode_disable_command() {
        let command = Commands::Scene(Scene::StudioModeDisable);

        match command {
            Commands::Scene(Scene::StudioModeDisable) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::StudioModeDisable command"),
        }
    }

    #[tokio::test]
    async fn test_scene_studio_mode_toggle_command() {
        let command = Commands::Scene(Scene::StudioModeToggle);

        match command {
            Commands::Scene(Scene::StudioModeToggle) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::StudioModeToggle command"),
        }
    }

    #[tokio::test]
    async fn test_scene_studio_mode_transition_command() {
        let command = Commands::Scene(Scene::StudioModeTransition);

        match command {
            Commands::Scene(Scene::StudioModeTransition) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::StudioModeTransition command"),
        }
    }

    #[tokio::test]
    async fn test_scene_preview_current_command() {
        let command = Commands::Scene(Scene::PreviewCurrent);

        match command {
            Commands::Scene(Scene::PreviewCurrent) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Scene::PreviewCurrent command"),
        }
    }

    #[tokio::test]
    async fn test_scene_preview_set_command() {
        let command = Commands::Scene(Scene::PreviewSet {
            scene_name: "preview_scene".to_string(),
        });

        match command {
            Commands::Scene(Scene::PreviewSet { scene_name }) => {
                assert_eq!(scene_name, "preview_scene");
            }
            _ => panic!("Expected Scene::PreviewSet command"),
        }
    }

    #[tokio::test]
    async fn test_recording_start_command() {
        let command = Commands::Recording(Recording::Start);

        match command {
            Commands::Recording(Recording::Start) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Recording::Start command"),
        }
    }

    #[tokio::test]
    async fn test_recording_stop_command() {
        let command = Commands::Recording(Recording::Stop);

        match command {
            Commands::Recording(Recording::Stop) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Recording::Stop command"),
        }
    }

    #[tokio::test]
    async fn test_recording_toggle_command() {
        let command = Commands::Recording(Recording::Toggle);

        match command {
            Commands::Recording(Recording::Toggle) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Recording::Toggle command"),
        }
    }

    #[tokio::test]
    async fn test_recording_status_command() {
        let command = Commands::Recording(Recording::Status);

        match command {
            Commands::Recording(Recording::Status) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Recording::Status command"),
        }
    }

    #[tokio::test]
    async fn test_streaming_start_command() {
        let command = Commands::Streaming(Streaming::Start);

        match command {
            Commands::Streaming(Streaming::Start) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Streaming::Start command"),
        }
    }

    #[tokio::test]
    async fn test_streaming_stop_command() {
        let command = Commands::Streaming(Streaming::Stop);

        match command {
            Commands::Streaming(Streaming::Stop) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Streaming::Stop command"),
        }
    }

    #[tokio::test]
    async fn test_streaming_status_command() {
        let command = Commands::Streaming(Streaming::Status);

        match command {
            Commands::Streaming(Streaming::Status) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Streaming::Status command"),
        }
    }

    #[tokio::test]
    async fn test_virtual_camera_start_command() {
        let command = Commands::VirtualCamera(VirtualCamera::Start);

        match command {
            Commands::VirtualCamera(VirtualCamera::Start) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected VirtualCamera::Start command"),
        }
    }

    #[tokio::test]
    async fn test_virtual_camera_stop_command() {
        let command = Commands::VirtualCamera(VirtualCamera::Stop);

        match command {
            Commands::VirtualCamera(VirtualCamera::Stop) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected VirtualCamera::Stop command"),
        }
    }

    #[tokio::test]
    async fn test_replay_start_command() {
        let command = Commands::Replay(Replay::Start);

        match command {
            Commands::Replay(Replay::Start) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Replay::Start command"),
        }
    }

    #[tokio::test]
    async fn test_replay_stop_command() {
        let command = Commands::Replay(Replay::Stop);

        match command {
            Commands::Replay(Replay::Stop) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Replay::Stop command"),
        }
    }

    #[tokio::test]
    async fn test_replay_save_command() {
        let command = Commands::Replay(Replay::Save);

        match command {
            Commands::Replay(Replay::Save) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Replay::Save command"),
        }
    }

    #[tokio::test]
    async fn test_audio_mute_command() {
        let command = Commands::Audio {
            command: "mute".to_string(),
            device: "Mic/Aux".to_string(),
        };

        match command {
            Commands::Audio { command, device } => {
                assert_eq!(command, "mute");
                assert_eq!(device, "Mic/Aux");
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[tokio::test]
    async fn test_audio_unmute_command() {
        let command = Commands::Audio {
            command: "unmute".to_string(),
            device: "Mic/Aux".to_string(),
        };

        match command {
            Commands::Audio { command, device } => {
                assert_eq!(command, "unmute");
                assert_eq!(device, "Mic/Aux");
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[tokio::test]
    async fn test_audio_toggle_command() {
        let command = Commands::Audio {
            command: "toggle".to_string(),
            device: "Mic/Aux".to_string(),
        };

        match command {
            Commands::Audio { command, device } => {
                assert_eq!(command, "toggle");
                assert_eq!(device, "Mic/Aux");
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[tokio::test]
    async fn test_filter_enable_command() {
        let command = Commands::Filter {
            command: "enable".to_string(),
            source: "Camera".to_string(),
            filter: "Color Correction".to_string(),
        };

        match command {
            Commands::Filter {
                command,
                source,
                filter,
            } => {
                assert_eq!(command, "enable");
                assert_eq!(source, "Camera");
                assert_eq!(filter, "Color Correction");
            }
            _ => panic!("Expected Filter command"),
        }
    }

    #[tokio::test]
    async fn test_scene_item_enable_command() {
        let command = Commands::SceneItem(SceneItem::Enable {
            scene: "Main Scene".to_string(),
            source: "Webcam".to_string(),
        });

        match command {
            Commands::SceneItem(SceneItem::Enable { scene, source }) => {
                assert_eq!(scene, "Main Scene");
                assert_eq!(source, "Webcam");
            }
            _ => panic!("Expected SceneItem::Enable command"),
        }
    }

    #[tokio::test]
    async fn test_trigger_hotkey_command() {
        let command = Commands::TriggerHotkey {
            name: "OBSBasic.StartRecording".to_string(),
        };

        match command {
            Commands::TriggerHotkey { name } => {
                assert_eq!(name, "OBSBasic.StartRecording");
            }
            _ => panic!("Expected TriggerHotkey command"),
        }
    }

    #[tokio::test]
    async fn test_info_command() {
        let command = Commands::Info;

        match command {
            Commands::Info => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Info command"),
        }
    }

    #[tokio::test]
    async fn test_save_screenshot_command() {
        let command = Commands::SaveScreenshot {
            source: "Camera".to_string(),
            format: "png".to_string(),
            file_path: PathBuf::from("/tmp/screenshot.png"),
            width: Some(1920),
            height: Some(1080),
            compression_quality: Some(80),
        };

        match command {
            Commands::SaveScreenshot {
                source,
                format,
                file_path,
                width,
                height,
                compression_quality,
            } => {
                assert_eq!(source, "Camera");
                assert_eq!(format, "png");
                assert_eq!(file_path, PathBuf::from("/tmp/screenshot.png"));
                assert_eq!(width, Some(1920));
                assert_eq!(height, Some(1080));
                assert_eq!(compression_quality, Some(80));
            }
            _ => panic!("Expected SaveScreenshot command"),
        }
    }

    #[tokio::test]
    async fn test_fullscreen_projector_command() {
        let command = Commands::FullscreenProjector { monitor_index: 1 };

        match command {
            Commands::FullscreenProjector { monitor_index } => {
                assert_eq!(monitor_index, 1);
            }
            _ => panic!("Expected FullscreenProjector command"),
        }
    }

    #[tokio::test]
    async fn test_source_projector_command() {
        let command = Commands::SourceProjector {
            name: "Camera".to_string(),
            monitor_index: 2,
        };

        match command {
            Commands::SourceProjector {
                name,
                monitor_index,
            } => {
                assert_eq!(name, "Camera");
                assert_eq!(monitor_index, 2);
            }
            _ => panic!("Expected SourceProjector command"),
        }
    }

    // Handler description tests (from original src/handlers/handler_tests.rs)
    #[tokio::test]
    async fn test_recording_handler_description() {
        let handler = RecordingHandler {
            action: Recording::Start,
        };
        assert_eq!(handler.description(), "Start recording");
    }

    #[tokio::test]
    async fn test_recording_create_chapter_command_with_name() {
        let chapter_name = Some("Test Chapter".to_string());
        let command = Commands::Recording(Recording::CreateChapter {
            chapter_name: chapter_name.clone(),
        });

        match command {
            Commands::Recording(Recording::CreateChapter { chapter_name: name }) => {
                assert_eq!(name, chapter_name);
            }
            _ => panic!("Expected Recording::CreateChapter command"),
        }
    }

    #[tokio::test]
    async fn test_recording_create_chapter_command_without_name() {
        let command = Commands::Recording(Recording::CreateChapter { chapter_name: None });

        match command {
            Commands::Recording(Recording::CreateChapter { chapter_name }) => {
                assert_eq!(chapter_name, None);
            }
            _ => panic!("Expected Recording::CreateChapter command"),
        }
    }

    #[tokio::test]
    async fn test_recording_create_chapter_handler_description() {
        let handler = RecordingHandler {
            action: Recording::CreateChapter {
                chapter_name: Some("Test Chapter".to_string()),
            },
        };
        assert_eq!(handler.description(), "Create a record chapter");
    }

    #[tokio::test]
    async fn test_streaming_handler_description() {
        let handler = StreamingHandler {
            action: Streaming::Toggle,
        };
        assert_eq!(handler.description(), "Toggle streaming");
    }

    #[tokio::test]
    async fn test_scene_handler_description() {
        let handler = SceneHandler {
            action: Scene::Current,
        };
        assert_eq!(handler.description(), "Get current scene");
    }

    #[tokio::test]
    async fn test_scene_create_handler_description() {
        let handler = SceneHandler {
            action: Scene::Create {
                scene_name: "test".to_string(),
            },
        };
        assert_eq!(handler.description(), "Create new scene");
    }

    #[tokio::test]
    async fn test_scene_transition_set_handler_description() {
        let handler = SceneHandler {
            action: Scene::TransitionSet {
                transition_name: "Fade".to_string(),
            },
        };
        assert_eq!(handler.description(), "Set current transition");
    }

    #[tokio::test]
    async fn test_scene_studio_mode_enable_handler_description() {
        let handler = SceneHandler {
            action: Scene::StudioModeEnable,
        };
        assert_eq!(handler.description(), "Enable studio mode");
    }

    #[tokio::test]
    async fn test_scene_preview_set_handler_description() {
        let handler = SceneHandler {
            action: Scene::PreviewSet {
                scene_name: "preview".to_string(),
            },
        };
        assert_eq!(handler.description(), "Set preview scene");
    }

    #[tokio::test]
    async fn test_media_input_handler_description() {
        let handler = MediaInputHandler {
            action: MediaInput::Play {
                name: "test".to_string(),
            },
        };
        assert_eq!(handler.description(), "Play media input");
    }

    #[tokio::test]
    async fn test_audio_handler_description() {
        let handler = AudioHandler {
            command: "mute".to_string(),
            device: "Mic/Aux".to_string(),
        };
        assert_eq!(handler.description(), "Mute audio device");
    }

    #[tokio::test]
    async fn test_filter_handler_description() {
        let handler = FilterHandler {
            command: "toggle".to_string(),
            source: "Camera".to_string(),
            filter: "Color Correction".to_string(),
        };
        assert_eq!(handler.description(), "Toggle filter");
    }

    #[tokio::test]
    async fn test_scene_collection_handler_description() {
        let handler = SceneCollectionHandler {
            action: SceneCollection::Switch {
                scene_collection_name: "Test Collection".to_string(),
            },
        };
        assert_eq!(handler.description(), "Switch to scene collection");
    }

    #[tokio::test]
    async fn test_virtual_camera_handler_description() {
        let handler = VirtualCameraHandler {
            action: VirtualCamera::Stop,
        };
        assert_eq!(handler.description(), "Stop virtual camera");
    }

    #[tokio::test]
    async fn test_info_handler_description() {
        let handler = InfoHandler;
        assert_eq!(handler.description(), "Get OBS version information");
    }

    #[tokio::test]
    async fn test_hotkey_handler_description() {
        let handler = HotkeyHandler {
            name: "OBSBasic.StartRecording".to_string(),
        };
        assert_eq!(handler.description(), "Trigger hotkey");
    }

    #[tokio::test]
    async fn test_source_handler_description() {
        let handler = SourceHandler {
            source: "Camera".to_string(),
            format: "png".to_string(),
            file_path: PathBuf::from("/tmp/screenshot.png"),
            width: Some(1920),
            height: Some(1080),
            compression_quality: Some(80),
        };
        assert_eq!(handler.description(), "Save source screenshot");
    }

    #[tokio::test]
    async fn test_fullscreen_projector_handler_description() {
        let handler = FullscreenProjectorHandler { monitor_index: 1 };
        assert_eq!(handler.description(), "Open fullscreen projector");
    }

    #[tokio::test]
    async fn test_source_projector_handler_description() {
        let handler = SourceProjectorHandler {
            name: "Camera".to_string(),
            monitor_index: 0,
        };
        assert_eq!(handler.description(), "Open source projector");
    }

    // Enhanced tests with actual handler execution logic
    #[tokio::test]
    async fn test_validate_monitor_index_valid() {
        let monitors = vec![
            obws::responses::ui::Monitor {
                index: 0,
                name: "Monitor 0".to_string(),
                position: obws::responses::ui::MonitorPosition { x: 0, y: 0 },
                size: obws::responses::ui::MonitorSize {
                    width: 1920,
                    height: 1080,
                },
            },
            obws::responses::ui::Monitor {
                index: 1,
                name: "Monitor 1".to_string(),
                position: obws::responses::ui::MonitorPosition { x: 1920, y: 0 },
                size: obws::responses::ui::MonitorSize {
                    width: 1920,
                    height: 1080,
                },
            },
        ];

        let result = validate_monitor_index(&monitors, 1);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_monitor_index_invalid() {
        let monitors = vec![obws::responses::ui::Monitor {
            index: 0,
            name: "Monitor 0".to_string(),
            position: obws::responses::ui::MonitorPosition { x: 0, y: 0 },
            size: obws::responses::ui::MonitorSize {
                width: 1920,
                height: 1080,
            },
        }];

        let result = validate_monitor_index(&monitors, 2);
        assert!(result.is_err());
    }

    // Input command tests
    #[tokio::test]
    async fn test_input_list_command() {
        let command = Commands::Input(Input::List { kind: None });

        match command {
            Commands::Input(Input::List { kind }) => {
                assert_eq!(kind, None);
            }
            _ => panic!("Expected Input::List command"),
        }
    }

    #[tokio::test]
    async fn test_input_list_with_kind_command() {
        let command = Commands::Input(Input::List {
            kind: Some("ffmpeg_source".to_string()),
        });

        match command {
            Commands::Input(Input::List { kind }) => {
                assert_eq!(kind, Some("ffmpeg_source".to_string()));
            }
            _ => panic!("Expected Input::List command with kind"),
        }
    }

    #[tokio::test]
    async fn test_input_list_kinds_command() {
        let command = Commands::Input(Input::ListKinds);

        match command {
            Commands::Input(Input::ListKinds) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Input::ListKinds command"),
        }
    }

    #[tokio::test]
    async fn test_input_create_command() {
        let command = Commands::Input(Input::Create {
            input_name: "test_input".to_string(),
            input_kind: "ffmpeg_source".to_string(),
            scene: None,
            settings: None,
        });

        match command {
            Commands::Input(Input::Create {
                input_name,
                input_kind,
                scene,
                settings,
            }) => {
                assert_eq!(input_name, "test_input");
                assert_eq!(input_kind, "ffmpeg_source");
                assert_eq!(scene, None);
                assert_eq!(settings, None);
            }
            _ => panic!("Expected Input::Create command"),
        }
    }

    #[tokio::test]
    async fn test_input_create_with_scene_and_settings_command() {
        let command = Commands::Input(Input::Create {
            input_name: "test_input".to_string(),
            input_kind: "image_source".to_string(),
            scene: Some("Main Scene".to_string()),
            settings: Some(r#"{"file": "/path/to/image.png"}"#.to_string()),
        });

        match command {
            Commands::Input(Input::Create {
                input_name,
                input_kind,
                scene,
                settings,
            }) => {
                assert_eq!(input_name, "test_input");
                assert_eq!(input_kind, "image_source");
                assert_eq!(scene, Some("Main Scene".to_string()));
                assert_eq!(
                    settings,
                    Some(r#"{"file": "/path/to/image.png"}"#.to_string())
                );
            }
            _ => panic!("Expected Input::Create command with scene and settings"),
        }
    }

    #[tokio::test]
    async fn test_input_remove_command() {
        let command = Commands::Input(Input::Remove {
            input_name: "old_input".to_string(),
        });

        match command {
            Commands::Input(Input::Remove { input_name }) => {
                assert_eq!(input_name, "old_input");
            }
            _ => panic!("Expected Input::Remove command"),
        }
    }

    #[tokio::test]
    async fn test_input_rename_command() {
        let command = Commands::Input(Input::Rename {
            input_name: "old_name".to_string(),
            new_name: "new_name".to_string(),
        });

        match command {
            Commands::Input(Input::Rename {
                input_name,
                new_name,
            }) => {
                assert_eq!(input_name, "old_name");
                assert_eq!(new_name, "new_name");
            }
            _ => panic!("Expected Input::Rename command"),
        }
    }

    #[tokio::test]
    async fn test_input_settings_get_command() {
        let command = Commands::Input(Input::Settings {
            input_name: "test_input".to_string(),
            get: true,
            set: None,
        });

        match command {
            Commands::Input(Input::Settings {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(get);
                assert_eq!(set, None);
            }
            _ => panic!("Expected Input::Settings get command"),
        }
    }

    #[tokio::test]
    async fn test_input_settings_set_command() {
        let command = Commands::Input(Input::Settings {
            input_name: "test_input".to_string(),
            get: false,
            set: Some(r#"{"key": "value"}"#.to_string()),
        });

        match command {
            Commands::Input(Input::Settings {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(!get);
                assert_eq!(set, Some(r#"{"key": "value"}"#.to_string()));
            }
            _ => panic!("Expected Input::Settings set command"),
        }
    }

    #[tokio::test]
    async fn test_input_volume_get_command() {
        let command = Commands::Input(Input::Volume {
            input_name: "test_input".to_string(),
            get: true,
            set: None,
        });

        match command {
            Commands::Input(Input::Volume {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(get);
                assert_eq!(set, None);
            }
            _ => panic!("Expected Input::Volume get command"),
        }
    }

    #[tokio::test]
    async fn test_input_volume_set_command() {
        let command = Commands::Input(Input::Volume {
            input_name: "test_input".to_string(),
            get: false,
            set: Some(0.75),
        });

        match command {
            Commands::Input(Input::Volume {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(!get);
                assert_eq!(set, Some(0.75));
            }
            _ => panic!("Expected Input::Volume set command"),
        }
    }

    #[tokio::test]
    async fn test_input_mute_mute_command() {
        let command = Commands::Input(Input::Mute {
            input_name: "test_input".to_string(),
            action: MuteAction::Mute,
        });

        match command {
            Commands::Input(Input::Mute { input_name, action }) => {
                assert_eq!(input_name, "test_input");
                assert_eq!(action, MuteAction::Mute);
            }
            _ => panic!("Expected Input::Mute mute command"),
        }
    }

    #[tokio::test]
    async fn test_input_mute_unmute_command() {
        let command = Commands::Input(Input::Mute {
            input_name: "test_input".to_string(),
            action: MuteAction::Unmute,
        });

        match command {
            Commands::Input(Input::Mute { input_name, action }) => {
                assert_eq!(input_name, "test_input");
                assert_eq!(action, MuteAction::Unmute);
            }
            _ => panic!("Expected Input::Mute unmute command"),
        }
    }

    #[tokio::test]
    async fn test_input_mute_toggle_command() {
        let command = Commands::Input(Input::Mute {
            input_name: "test_input".to_string(),
            action: MuteAction::Toggle,
        });

        match command {
            Commands::Input(Input::Mute { input_name, action }) => {
                assert_eq!(input_name, "test_input");
                assert_eq!(action, MuteAction::Toggle);
            }
            _ => panic!("Expected Input::Mute toggle command"),
        }
    }

    #[tokio::test]
    async fn test_input_mute_status_command() {
        let command = Commands::Input(Input::Mute {
            input_name: "test_input".to_string(),
            action: MuteAction::Status,
        });

        match command {
            Commands::Input(Input::Mute { input_name, action }) => {
                assert_eq!(input_name, "test_input");
                assert_eq!(action, MuteAction::Status);
            }
            _ => panic!("Expected Input::Mute status command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_balance_get_command() {
        let command = Commands::Input(Input::AudioBalance {
            input_name: "test_input".to_string(),
            get: true,
            set: None,
        });

        match command {
            Commands::Input(Input::AudioBalance {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(get);
                assert_eq!(set, None);
            }
            _ => panic!("Expected Input::AudioBalance get command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_balance_set_command() {
        let command = Commands::Input(Input::AudioBalance {
            input_name: "test_input".to_string(),
            get: false,
            set: Some(0.5),
        });

        match command {
            Commands::Input(Input::AudioBalance {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(!get);
                assert_eq!(set, Some(0.5));
            }
            _ => panic!("Expected Input::AudioBalance set command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_sync_offset_get_command() {
        let command = Commands::Input(Input::AudioSyncOffset {
            input_name: "test_input".to_string(),
            get: true,
            set: None,
        });

        match command {
            Commands::Input(Input::AudioSyncOffset {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(get);
                assert_eq!(set, None);
            }
            _ => panic!("Expected Input::AudioSyncOffset get command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_sync_offset_set_command() {
        let command = Commands::Input(Input::AudioSyncOffset {
            input_name: "test_input".to_string(),
            get: false,
            set: Some(1000000),
        });

        match command {
            Commands::Input(Input::AudioSyncOffset {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(!get);
                assert_eq!(set, Some(1000000));
            }
            _ => panic!("Expected Input::AudioSyncOffset set command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_monitor_type_get_command() {
        let command = Commands::Input(Input::AudioMonitorType {
            input_name: "test_input".to_string(),
            get: true,
            set: None,
        });

        match command {
            Commands::Input(Input::AudioMonitorType {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(get);
                assert_eq!(set, None);
            }
            _ => panic!("Expected Input::AudioMonitorType get command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_monitor_type_set_command() {
        let command = Commands::Input(Input::AudioMonitorType {
            input_name: "test_input".to_string(),
            get: false,
            set: Some("monitorOnly".to_string()),
        });

        match command {
            Commands::Input(Input::AudioMonitorType {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(!get);
                assert_eq!(set, Some("monitorOnly".to_string()));
            }
            _ => panic!("Expected Input::AudioMonitorType set command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_tracks_get_command() {
        let command = Commands::Input(Input::AudioTracks {
            input_name: "test_input".to_string(),
            get: true,
            set: None,
        });

        match command {
            Commands::Input(Input::AudioTracks {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(get);
                assert_eq!(set, None);
            }
            _ => panic!("Expected Input::AudioTracks get command"),
        }
    }

    #[tokio::test]
    async fn test_input_audio_tracks_set_command() {
        let command = Commands::Input(Input::AudioTracks {
            input_name: "test_input".to_string(),
            get: false,
            set: Some(r#"{"1": true, "2": false}"#.to_string()),
        });

        match command {
            Commands::Input(Input::AudioTracks {
                input_name,
                get,
                set,
            }) => {
                assert_eq!(input_name, "test_input");
                assert!(!get);
                assert_eq!(set, Some(r#"{"1": true, "2": false}"#.to_string()));
            }
            _ => panic!("Expected Input::AudioTracks set command"),
        }
    }

    #[tokio::test]
    async fn test_input_default_settings_command() {
        let command = Commands::Input(Input::DefaultSettings {
            input_kind: "ffmpeg_source".to_string(),
        });

        match command {
            Commands::Input(Input::DefaultSettings { input_kind }) => {
                assert_eq!(input_kind, "ffmpeg_source");
            }
            _ => panic!("Expected Input::DefaultSettings command"),
        }
    }

    #[tokio::test]
    async fn test_input_specials_command() {
        let command = Commands::Input(Input::Specials);

        match command {
            Commands::Input(Input::Specials) => {
                // Test passes if pattern matches
            }
            _ => panic!("Expected Input::Specials command"),
        }
    }

    // Input handler description tests
    #[tokio::test]
    async fn test_input_handler_description_list() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::List { kind: None },
        };
        assert_eq!(handler.description(), "List inputs");
    }

    #[tokio::test]
    async fn test_input_handler_description_list_kinds() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::ListKinds,
        };
        assert_eq!(handler.description(), "List available input kinds");
    }

    #[tokio::test]
    async fn test_input_handler_description_create() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::Create {
                input_name: "test".to_string(),
                input_kind: "ffmpeg_source".to_string(),
                scene: None,
                settings: None,
            },
        };
        assert_eq!(handler.description(), "Create new input");
    }

    #[tokio::test]
    async fn test_input_handler_description_remove() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::Remove {
                input_name: "test".to_string(),
            },
        };
        assert_eq!(handler.description(), "Remove input");
    }

    #[tokio::test]
    async fn test_input_handler_description_rename() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::Rename {
                input_name: "old".to_string(),
                new_name: "new".to_string(),
            },
        };
        assert_eq!(handler.description(), "Rename input");
    }

    #[tokio::test]
    async fn test_input_handler_description_settings() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::Settings {
                input_name: "test".to_string(),
                get: true,
                set: None,
            },
        };
        assert_eq!(handler.description(), "Manage input settings");
    }

    #[tokio::test]
    async fn test_input_handler_description_volume() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::Volume {
                input_name: "test".to_string(),
                get: false,
                set: Some(0.5),
            },
        };
        assert_eq!(handler.description(), "Manage input volume");
    }

    #[tokio::test]
    async fn test_input_handler_description_mute() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::Mute {
                input_name: "test".to_string(),
                action: MuteAction::Toggle,
            },
        };
        assert_eq!(handler.description(), "Manage input mute state");
    }

    #[tokio::test]
    async fn test_input_handler_description_audio_balance() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::AudioBalance {
                input_name: "test".to_string(),
                get: true,
                set: None,
            },
        };
        assert_eq!(handler.description(), "Manage audio balance");
    }

    #[tokio::test]
    async fn test_input_handler_description_audio_sync_offset() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::AudioSyncOffset {
                input_name: "test".to_string(),
                get: false,
                set: Some(1000),
            },
        };
        assert_eq!(handler.description(), "Manage audio sync offset");
    }

    #[tokio::test]
    async fn test_input_handler_description_audio_monitor_type() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::AudioMonitorType {
                input_name: "test".to_string(),
                get: true,
                set: None,
            },
        };
        assert_eq!(handler.description(), "Manage audio monitor type");
    }

    #[tokio::test]
    async fn test_input_handler_description_audio_tracks() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::AudioTracks {
                input_name: "test".to_string(),
                get: false,
                set: Some(r#"{"1": true}"#.to_string()),
            },
        };
        assert_eq!(handler.description(), "Manage audio tracks");
    }

    #[tokio::test]
    async fn test_input_handler_description_default_settings() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::DefaultSettings {
                input_kind: "ffmpeg_source".to_string(),
            },
        };
        assert_eq!(handler.description(), "Get default settings for input kind");
    }

    #[tokio::test]
    async fn test_input_handler_description_specials() {
        use crate::handlers::inputs::InputCmdHandler;
        let handler = InputCmdHandler {
            action: Input::Specials,
        };
        assert_eq!(handler.description(), "List special inputs");
    }

    // Integration tests would go here in a real implementation
    // These would test the actual handler logic with mock OBS responses
}

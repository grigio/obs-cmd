#[cfg(test)]
mod tests {
    use crate::cli::{MediaInput, Recording, Scene, SceneCollection, Streaming, VirtualCamera};
    use crate::handlers::{
        audio::AudioHandler, filters::FilterHandler, general::HotkeyHandler, general::InfoHandler,
        media::MediaInputHandler, recording::RecordingHandler,
        scene_collections::SceneCollectionHandler, scenes::SceneHandler, sources::SourceHandler,
        streaming::StreamingHandler, ui::FullscreenProjectorHandler, ui::SourceProjectorHandler,
        virtual_camera::VirtualCameraHandler, CommandHandler,
    };
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_recording_handler_description() {
        let handler = RecordingHandler {
            action: Recording::Start,
        };
        assert_eq!(handler.description(), "Start recording");
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
}

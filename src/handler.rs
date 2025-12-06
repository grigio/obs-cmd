use crate::cli::Commands;
use crate::connection::check_connection_health;
use crate::error::Result;
use crate::handlers::{
    audio::AudioHandler, config::ProfileHandler, config::RecordDirectoryHandler,
    config::StreamServiceHandler, config::VideoSettingsHandler, filters::FilterHandler,
    general::HotkeyHandler, general::InfoHandler, media::MediaInputHandler,
    recording::RecordingHandler, replay_buffer::ReplayBufferHandler,
    scene_collections::SceneCollectionHandler, scene_items::SceneItemHandler, scenes::SceneHandler,
    sources::SourceHandler, streaming::StreamingHandler, ui::FullscreenProjectorHandler,
    ui::SourceProjectorHandler, virtual_camera::VirtualCameraHandler, CommandHandler,
};
use obws::Client;

/// Handles all OBS WebSocket commands and routes them to appropriate handlers.
///
/// This function is the main command dispatcher that takes a client connection
/// and a command enum, then executes the corresponding handler.
/// It includes connection health checking and comprehensive error handling.
pub async fn handle_commands(client: &Client, commands: &Commands) -> Result<()> {
    // Check connection health before executing commands
    if let Err(e) = check_connection_health(client).await {
        eprintln!("Warning: Connection health check failed: {}", e);
        // Continue with command execution but warn user
    }

    let handler: Box<dyn CommandHandler> = match commands {
        Commands::Info => Box::new(InfoHandler),
        Commands::MediaInput(media_input) => Box::new(MediaInputHandler {
            action: media_input.clone(),
        }),
        Commands::Scene(action) => Box::new(SceneHandler {
            action: action.clone(),
        }),
        Commands::SceneCollection(action) => Box::new(SceneCollectionHandler {
            action: action.clone(),
        }),
        Commands::Profile(action) => Box::new(ProfileHandler {
            action: action.clone(),
        }),
        Commands::VideoSettings(action) => Box::new(VideoSettingsHandler {
            action: action.clone(),
        }),
        Commands::StreamService(action) => Box::new(StreamServiceHandler {
            action: action.clone(),
        }),
        Commands::RecordDirectory(action) => Box::new(RecordDirectoryHandler {
            action: action.clone(),
        }),
        Commands::Recording(action) => Box::new(RecordingHandler {
            action: action.clone(),
        }),
        Commands::Streaming(action) => Box::new(StreamingHandler {
            action: action.clone(),
        }),
        Commands::VirtualCamera(action) => Box::new(VirtualCameraHandler {
            action: action.clone(),
        }),
        Commands::Replay(action) => Box::new(ReplayBufferHandler {
            action: action.clone(),
        }),
        Commands::Audio { command, device } => Box::new(AudioHandler {
            command: command.clone(),
            device: device.clone(),
        }),
        Commands::Filter {
            command,
            source,
            filter,
        } => Box::new(FilterHandler {
            command: command.clone(),
            source: source.clone(),
            filter: filter.clone(),
        }),
        Commands::SceneItem(action) => Box::new(SceneItemHandler {
            action: action.clone(),
        }),
        Commands::TriggerHotkey { name } => Box::new(HotkeyHandler { name: name.clone() }),
        Commands::FullscreenProjector { monitor_index } => Box::new(FullscreenProjectorHandler {
            monitor_index: *monitor_index,
        }),
        Commands::SourceProjector {
            name,
            monitor_index,
        } => Box::new(SourceProjectorHandler {
            name: name.clone(),
            monitor_index: *monitor_index,
        }),
        Commands::SaveScreenshot {
            source,
            format,
            file_path,
            width,
            height,
            compression_quality,
        } => Box::new(SourceHandler {
            source: source.clone(),
            format: format.clone(),
            file_path: file_path.clone(),
            width: *width,
            height: *height,
            compression_quality: *compression_quality,
        }),
        Commands::Completion { .. } => {
            // This should never reach here as completion is handled in main()
            panic!("Completion command should be handled in main()");
        }
    };

    println!("Executing: {}", handler.description());
    handler.execute(client).await
}

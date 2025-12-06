use crate::cli::{Profile, VideoSettings, StreamService, RecordDirectory};
use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for profile-related commands
pub struct ProfileHandler {
    pub action: Profile,
}

#[async_trait::async_trait]
impl CommandHandler for ProfileHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            Profile::Current => {
                let profile_name = client
                    .profiles()
                    .current()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Current profile: {}", profile_name);
            }
            Profile::List => {
                let profiles = client
                    .profiles()
                    .list()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Available profiles:");
                for profile in profiles.profiles {
                    println!("  - {}", profile);
                }
            }
            Profile::Create { profile_name } => {
                println!("Creating profile: {}", profile_name);
                client
                    .profiles()
                    .create(profile_name)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Profile created successfully");
            }
            Profile::Remove { profile_name } => {
                println!("Removing profile: {}", profile_name);
                client
                    .profiles()
                    .remove(profile_name)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Profile removed successfully");
            }
            Profile::Switch { profile_name } => {
                println!("Switching to profile: {}", profile_name);
                client
                    .profiles()
                    .set_current(profile_name)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Profile switched successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            Profile::Current => "Get current profile",
            Profile::List => "List available profiles",
            Profile::Create { .. } => "Create new profile",
            Profile::Remove { .. } => "Remove profile",
            Profile::Switch { .. } => "Switch to profile",
        }
    }
}

/// Handler for video settings commands
pub struct VideoSettingsHandler {
    pub action: VideoSettings,
}

#[async_trait::async_trait]
impl CommandHandler for VideoSettingsHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            VideoSettings::Get => {
                let settings = client
                    .config()
                    .video_settings()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Video Settings:");
                println!("  Base Resolution: {}x{}", settings.base_width, settings.base_height);
                println!("  Output Resolution: {}x{}", settings.output_width, settings.output_height);
                println!("  FPS: {}/{}", settings.fps_numerator, settings.fps_denominator);
            }
            VideoSettings::Set {
                base_width,
                base_height,
                output_width,
                output_height,
                fps_num,
                fps_den,
            } => {
                use obws::requests::config::SetVideoSettings;
                
                let mut settings = SetVideoSettings::default();
                
                if let (Some(width), Some(height)) = (base_width, base_height) {
                    settings.base_width = Some(*width);
                    settings.base_height = Some(*height);
                }
                
                if let (Some(width), Some(height)) = (output_width, output_height) {
                    settings.output_width = Some(*width);
                    settings.output_height = Some(*height);
                }
                
                if let (Some(num), Some(den)) = (fps_num, fps_den) {
                    settings.fps_numerator = Some(*num);
                    settings.fps_denominator = Some(*den);
                }
                
                println!("Updating video settings...");
                client
                    .config()
                    .set_video_settings(settings)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Video settings updated successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            VideoSettings::Get => "Get current video settings",
            VideoSettings::Set { .. } => "Set video settings",
        }
    }
}

/// Handler for stream service settings commands
pub struct StreamServiceHandler {
    pub action: StreamService,
}

#[async_trait::async_trait]
impl CommandHandler for StreamServiceHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            StreamService::Get => {
                let settings = client
                    .config()
                    .stream_service_settings::<serde_json::Value>()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Stream Service Settings:");
                println!("  Type: {}", settings.r#type);
                if let Some(server) = settings.settings.get("server") {
                    println!("  Server: {}", server);
                }
                if let Some(key) = settings.settings.get("key") {
                    println!("  Key: {}", key);
                }
            }
            StreamService::Set {
                service_type,
                server,
                key,
            } => {
                use serde_json::json;
                
                let mut settings = json!({});
                
                if let Some(srv) = server {
                    settings["server"] = json!(srv);
                }
                
                if let Some(k) = key {
                    settings["key"] = json!(k);
                }
                
                println!("Updating stream service settings...");
                client
                    .config()
                    .set_stream_service_settings(service_type, &settings)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Stream service settings updated successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            StreamService::Get => "Get current stream service settings",
            StreamService::Set { .. } => "Set stream service settings",
        }
    }
}

/// Handler for record directory commands
pub struct RecordDirectoryHandler {
    pub action: RecordDirectory,
}

#[async_trait::async_trait]
impl CommandHandler for RecordDirectoryHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            RecordDirectory::Get => {
                let directory = client
                    .config()
                    .record_directory()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Current record directory: {}", directory);
            }
            RecordDirectory::Set { directory } => {
                println!("Setting record directory to: {}", directory);
                client
                    .config()
                    .set_record_directory(directory)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Record directory set successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            RecordDirectory::Get => "Get current record directory",
            RecordDirectory::Set { .. } => "Set record directory",
        }
    }
}
use crate::cli::Scene;
use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::Client;
use time::Duration;

/// Handler for scene-related commands
pub struct SceneHandler {
    pub action: Scene,
}

#[async_trait::async_trait]
impl CommandHandler for SceneHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            // Basic scene controls
            Scene::Current => {
                let scene = client
                    .scenes()
                    .current_program_scene()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Current scene: {}", scene.id.name);
            }
            Scene::Switch { scene_name } => {
                println!("Switching to scene: {}", scene_name);
                client
                    .scenes()
                    .set_current_program_scene(scene_name.as_str())
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Scene switched successfully");
            }
            Scene::List => {
                let scenes = client
                    .scenes()
                    .list()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Available scenes:");
                for scene in scenes.scenes {
                    println!("  - {}", scene.id.name);
                }
            }
            Scene::Create { scene_name } => {
                println!("Creating scene: {}", scene_name);
                client
                    .scenes()
                    .create(scene_name.as_str())
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Scene created successfully");
            }
            Scene::Remove { scene_name } => {
                println!("Removing scene: {}", scene_name);
                client
                    .scenes()
                    .remove(obws::requests::scenes::SceneId::Name(scene_name.as_str()))
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Scene removed successfully");
            }
            Scene::Rename { scene_name, new_name } => {
                println!("Renaming scene '{}' to '{}'", scene_name, new_name);
                client
                    .scenes()
                    .set_name(obws::requests::scenes::SceneId::Name(scene_name.as_str()), new_name.as_str())
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Scene renamed successfully");
            }

            // Transition controls
            Scene::TransitionList => {
                let transitions = client
                    .transitions()
                    .list()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Available transitions:");
                for transition in transitions.transitions {
                    println!("  - {}", transition.id.name);
                }
            }
            Scene::TransitionCurrent => {
                let current = client
                    .transitions()
                    .current()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Current transition: {}", current.id.name);
                if let Some(duration) = current.duration {
                    println!("Duration: {}ms", duration.whole_milliseconds());
                }
            }
            Scene::TransitionSet { transition_name } => {
                println!("Setting transition to: {}", transition_name);
                client
                    .transitions()
                    .set_current(transition_name.as_str())
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Transition set successfully");
            }
            Scene::TransitionDuration { duration_ms } => {
                let duration = Duration::milliseconds(*duration_ms as i64);
                println!("Setting transition duration to: {}ms", duration_ms);
                client
                    .transitions()
                    .set_current_duration(duration)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Transition duration set successfully");
            }
            Scene::TransitionTrigger => {
                println!("Triggering transition");
                client
                    .transitions()
                    .trigger()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Transition triggered successfully");
            }

            // Studio mode controls
            Scene::StudioModeStatus => {
                let studio_mode_enabled = client
                    .ui()
                    .studio_mode_enabled()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Studio mode: {}", if studio_mode_enabled { "enabled" } else { "disabled" });
            }
            Scene::StudioModeEnable => {
                println!("Enabling studio mode");
                client
                    .ui()
                    .set_studio_mode_enabled(true)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Studio mode enabled");
            }
            Scene::StudioModeDisable => {
                println!("Disabling studio mode");
                client
                    .ui()
                    .set_studio_mode_enabled(false)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Studio mode disabled");
            }
            Scene::StudioModeToggle => {
                println!("Toggling studio mode");
                let current = client
                    .ui()
                    .studio_mode_enabled()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                client
                    .ui()
                    .set_studio_mode_enabled(!current)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Studio mode toggled to: {}", if !current { "enabled" } else { "disabled" });
            }
            Scene::StudioModeTransition => {
                println!("Triggering studio mode transition");
                client
                    .transitions()
                    .trigger()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Studio mode transition triggered");
            }

            // Preview scene controls (studio mode only)
            Scene::PreviewCurrent => {
                let preview_scene = client
                    .scenes()
                    .current_preview_scene()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Current preview scene: {}", preview_scene.id.name);
            }
            Scene::PreviewSet { scene_name } => {
                println!("Setting preview scene to: {}", scene_name);
                client
                    .scenes()
                    .set_current_preview_scene(scene_name.as_str())
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Preview scene set successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            Scene::Current => "Get current scene",
            Scene::Switch { .. } => "Switch to scene",
            Scene::List => "List all scenes",
            Scene::Create { .. } => "Create new scene",
            Scene::Remove { .. } => "Remove scene",
            Scene::Rename { .. } => "Rename scene",
            Scene::TransitionList => "List available transitions",
            Scene::TransitionCurrent => "Get current transition",
            Scene::TransitionSet { .. } => "Set current transition",
            Scene::TransitionDuration { .. } => "Set transition duration",
            Scene::TransitionTrigger => "Trigger transition",
            Scene::StudioModeStatus => "Get studio mode status",
            Scene::StudioModeEnable => "Enable studio mode",
            Scene::StudioModeDisable => "Disable studio mode",
            Scene::StudioModeToggle => "Toggle studio mode",
            Scene::StudioModeTransition => "Trigger studio mode transition",
            Scene::PreviewCurrent => "Get current preview scene",
            Scene::PreviewSet { .. } => "Set preview scene",
        }
    }
}

use crate::cli::Scene;
use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for scene-related commands
pub struct SceneHandler {
    pub action: Scene,
}

#[async_trait::async_trait]
impl CommandHandler for SceneHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            Scene::Current => {
                let scene_name = client
                    .scenes()
                    .current_program_scene()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Current scene: {:?}", scene_name);
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
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            Scene::Current => "Get current scene",
            Scene::Switch { .. } => "Switch to scene",
        }
    }
}

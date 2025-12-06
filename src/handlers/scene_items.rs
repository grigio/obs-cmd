use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::requests::scene_items::Id as IdItem;
use obws::requests::scene_items::SetEnabled as SetEnabledItem;
use obws::Client;

/// Handler for scene item enable/disable commands
pub struct SceneItemHandler {
    pub command: String,
    pub scene: String,
    pub source: String,
}

#[async_trait::async_trait]
impl CommandHandler for SceneItemHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        // Get scene item ID
        let item_id = client
            .scene_items()
            .id(IdItem {
                scene: self.scene.as_str().into(),
                source: self.source.as_str(),
                search_offset: Some(0),
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))?;

        let enabled: bool = match self.command.as_str() {
            "enable" => true,
            "disable" => false,
            "toggle" => {
                let current_state = client
                    .scene_items()
                    .enabled(self.scene.as_str().into(), item_id)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                !current_state
            }
            _ => {
                return Err(ObsCmdError::InvalidSceneItemCommand {
                    command: self.command.clone(),
                });
            }
        };

        client
            .scene_items()
            .set_enabled(SetEnabledItem {
                scene: self.scene.as_str().into(),
                item_id,
                enabled,
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))?;

        println!(
            "Scene item '{}' in scene '{}': {}",
            self.source,
            self.scene,
            if enabled { "enabled" } else { "disabled" }
        );
        println!("Scene item operation completed successfully");
        Ok(())
    }

    fn description(&self) -> &'static str {
        match self.command.as_str() {
            "enable" => "Enable scene item",
            "disable" => "Disable scene item",
            "toggle" => "Toggle scene item",
            _ => "Scene item operation",
        }
    }
}

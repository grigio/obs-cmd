use crate::cli::SceneCollection;
use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for scene collection-related commands
pub struct SceneCollectionHandler {
    pub action: SceneCollection,
}

#[async_trait::async_trait]
impl CommandHandler for SceneCollectionHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            SceneCollection::Current => {
                let scene_collection_name = client
                    .scene_collections()
                    .current()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Current scene collection: {}", scene_collection_name);
            }
            SceneCollection::List => {
                let scene_collections = client
                    .scene_collections()
                    .list()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Available scene collections:");
                for collection in scene_collections.collections {
                    println!("  - {}", collection);
                }
            }
            SceneCollection::Create {
                scene_collection_name,
            } => {
                println!("Creating scene collection: {}", scene_collection_name);
                client
                    .scene_collections()
                    .create(scene_collection_name)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Scene collection created successfully");
            }
            SceneCollection::Switch {
                scene_collection_name,
            } => {
                println!("Switching to scene collection: {}", scene_collection_name);
                client
                    .scene_collections()
                    .set_current(scene_collection_name)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Scene collection switched successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            SceneCollection::Current => "Get current scene collection",
            SceneCollection::List => "List available scene collections",
            SceneCollection::Create { .. } => "Create new scene collection",
            SceneCollection::Switch { .. } => "Switch to scene collection",
        }
    }
}

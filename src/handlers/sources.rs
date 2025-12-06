use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::requests::sources::SaveScreenshot;
use obws::Client;
use std::path::PathBuf;

/// Handler for screenshot commands
pub struct SourceHandler {
    pub source: String,
    pub format: String,
    pub file_path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub compression_quality: Option<i32>,
}

#[async_trait::async_trait]
impl CommandHandler for SourceHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        let settings = SaveScreenshot {
            source: obws::requests::sources::SourceId::Name(self.source.as_str()),
            format: &self.format,
            width: self.width,
            height: self.height,
            compression_quality: self.compression_quality,
            file_path: &self.file_path,
        };

        client
            .sources()
            .save_screenshot(settings)
            .await
            .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
        println!("Saved screenshot to path: {:?}", self.file_path);
        println!("Screenshot operation completed successfully");
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Save source screenshot"
    }
}

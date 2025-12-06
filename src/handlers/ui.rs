use crate::error::Result;
use crate::handlers::{validate_monitor_index, CommandHandler};
use obws::requests::ui::Location::MonitorIndex as MonitorLocationIndex;
use obws::requests::ui::OpenSourceProjector;
use obws::requests::ui::OpenVideoMixProjector;
use obws::requests::ui::VideoMixType::Program as OpenVideoMixProjectorType;
use obws::Client;

/// Handler for fullscreen projector commands
pub struct FullscreenProjectorHandler {
    pub monitor_index: u8,
}

#[async_trait::async_trait]
impl CommandHandler for FullscreenProjectorHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        let monitor_list = client
            .ui()
            .list_monitors()
            .await
            .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
        validate_monitor_index(&monitor_list, self.monitor_index)?;

        client
            .ui()
            .open_video_mix_projector(OpenVideoMixProjector {
                r#type: OpenVideoMixProjectorType,
                location: Some(MonitorLocationIndex(self.monitor_index as i32)),
            })
            .await
            .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;

        println!(
            "Opened fullscreen projector on monitor {}",
            self.monitor_index
        );
        println!("Projector operation completed successfully");
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Open fullscreen projector"
    }
}

/// Handler for source projector commands
pub struct SourceProjectorHandler {
    pub name: String,
    pub monitor_index: u8,
}

#[async_trait::async_trait]
impl CommandHandler for SourceProjectorHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        let monitor_list = client
            .ui()
            .list_monitors()
            .await
            .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
        validate_monitor_index(&monitor_list, self.monitor_index)?;

        client
            .ui()
            .open_source_projector(OpenSourceProjector {
                source: self.name.as_str().into(),
                location: Some(MonitorLocationIndex(self.monitor_index as i32)),
            })
            .await
            .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;

        println!(
            "Opened source projector for '{}' on monitor {}",
            self.name, self.monitor_index
        );
        println!("Projector operation completed successfully");
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Open source projector"
    }
}

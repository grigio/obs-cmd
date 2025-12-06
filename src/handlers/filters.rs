use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::requests::filters::SetEnabled as SetEnabledFilter;
use obws::Client;

/// Handler for filter enable/disable commands
pub struct FilterHandler {
    pub command: String,
    pub source: String,
    pub filter: String,
}

#[async_trait::async_trait]
impl CommandHandler for FilterHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        let enabled: bool = match self.command.as_str() {
            "enable" => true,
            "disable" => false,
            "toggle" => {
                let current_state = client
                    .filters()
                    .get(self.source.as_str().into(), &self.filter)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?
                    .enabled;
                !current_state
            }
            _ => {
                return Err(ObsCmdError::InvalidFilterCommand {
                    command: self.command.clone(),
                });
            }
        };

        client
            .filters()
            .set_enabled(SetEnabledFilter {
                source: self.source.as_str().into(),
                filter: &self.filter,
                enabled,
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))?;

        println!(
            "Filter '{}' on source '{}': {}",
            self.filter,
            self.source,
            if enabled { "enabled" } else { "disabled" }
        );
        println!("Filter operation completed successfully");
        Ok(())
    }

    fn description(&self) -> &'static str {
        match self.command.as_str() {
            "enable" => "Enable filter",
            "disable" => "Disable filter",
            "toggle" => "Toggle filter",
            _ => "Filter operation",
        }
    }
}

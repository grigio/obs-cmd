use crate::cli::MediaInput;
use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::common::MediaAction;
use obws::Client;

/// Handler for media input-related commands
pub struct MediaInputHandler {
    pub action: MediaInput,
}

#[async_trait::async_trait]
impl CommandHandler for MediaInputHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            MediaInput::SetCursor { name, cursor } => {
                println!("Setting cursor for media input '{}' to: {:?}", name, cursor);
                client
                    .media_inputs()
                    .set_cursor(name.as_str().into(), *cursor)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Media input cursor set successfully");
            }
            MediaInput::Play { name } => {
                println!("Playing media input: {}", name);
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Play)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Media input playing successfully");
            }
            MediaInput::Restart { name } => {
                println!("Restarting media input: {}", name);
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Restart)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Media input restarted successfully");
            }
            MediaInput::Pause { name } => {
                println!("Pausing media input: {}", name);
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Pause)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Media input paused successfully");
            }
            MediaInput::Stop { name } => {
                println!("Stopping media input: {}", name);
                client
                    .media_inputs()
                    .trigger_action(name.as_str().into(), MediaAction::Stop)
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Media input stopped successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            MediaInput::SetCursor { .. } => "Set media input cursor",
            MediaInput::Play { .. } => "Play media input",
            MediaInput::Restart { .. } => "Restart media input",
            MediaInput::Pause { .. } => "Pause media input",
            MediaInput::Stop { .. } => "Stop media input",
        }
    }
}

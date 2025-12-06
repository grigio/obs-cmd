use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for audio mute commands
pub struct AudioHandler {
    pub command: String,
    pub device: String,
}

#[async_trait::async_trait]
impl CommandHandler for AudioHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match self.command.as_str() {
            "mute" => {
                println!("Muting audio device: {}", self.device);
                client
                    .inputs()
                    .set_muted(self.device.as_str().into(), true)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Audio device muted successfully");
            }
            "unmute" => {
                println!("Unmuting audio device: {}", self.device);
                client
                    .inputs()
                    .set_muted(self.device.as_str().into(), false)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Audio device unmuted successfully");
            }
            "toggle" => {
                let current_state = client
                    .inputs()
                    .muted(self.device.as_str().into())
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                let new_state = !current_state;
                println!(
                    "Toggling audio device {}: {} -> {}",
                    self.device,
                    if current_state { "muted" } else { "unmuted" },
                    if new_state { "muted" } else { "unmuted" }
                );
                client
                    .inputs()
                    .set_muted(self.device.as_str().into(), new_state)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Audio device toggled successfully");
            }
            "status" => {
                let status = client
                    .inputs()
                    .muted(self.device.as_str().into())
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!(
                    "Audio device '{}' is: {}",
                    self.device,
                    if status { "muted" } else { "unmuted" }
                );
            }
            _ => {
                return Err(ObsCmdError::InvalidAudioCommand {
                    command: self.command.clone(),
                });
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match self.command.as_str() {
            "mute" => "Mute audio device",
            "unmute" => "Unmute audio device",
            "toggle" => "Toggle audio device mute state",
            "status" => "Get audio device status",
            _ => "Audio operation",
        }
    }
}

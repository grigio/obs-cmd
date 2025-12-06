use crate::cli::Replay;
use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for replay buffer commands
pub struct ReplayBufferHandler {
    pub action: Replay,
}

#[async_trait::async_trait]
impl CommandHandler for ReplayBufferHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            Replay::Start => {
                println!("Starting replay buffer...");
                client
                    .replay_buffer()
                    .start()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Replay buffer started successfully");
            }
            Replay::Stop => {
                println!("Stopping replay buffer...");
                client
                    .replay_buffer()
                    .stop()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Replay buffer stopped successfully");
            }
            Replay::Toggle => {
                println!("Toggling replay buffer...");
                client
                    .replay_buffer()
                    .toggle()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Replay buffer toggled successfully");
            }
            Replay::Save => {
                println!("Saving replay buffer...");
                client
                    .replay_buffer()
                    .save()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Replay buffer saved successfully");
            }
            Replay::Status => {
                let status = client
                    .replay_buffer()
                    .status()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!(
                    "Replay Buffer is {}",
                    if status { "running" } else { "not running" }
                );
            }
            Replay::LastReplay => {
                let res = client
                    .replay_buffer()
                    .last_replay()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                if res.is_empty() {
                    return Err(ObsCmdError::NoLastReplay);
                }
                println!("Last replay path: {}", res);
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            Replay::Start => "Start replay buffer",
            Replay::Stop => "Stop replay buffer",
            Replay::Toggle => "Toggle replay buffer",
            Replay::Save => "Save replay buffer",
            Replay::Status => "Get replay buffer status",
            Replay::LastReplay => "Get last replay path",
        }
    }
}

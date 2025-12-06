use crate::cli::Recording;
use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for recording-related commands
pub struct RecordingHandler {
    pub action: Recording,
}

#[async_trait::async_trait]
impl CommandHandler for RecordingHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            Recording::Start => {
                println!("Starting recording...");
                client
                    .recording()
                    .start()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Recording started successfully");
            }
            Recording::Stop => {
                println!("Stopping recording...");
                client
                    .recording()
                    .stop()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Recording stopped successfully");
            }
            Recording::Toggle => {
                println!("Toggling recording...");
                client
                    .recording()
                    .toggle()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Recording toggled successfully");
            }
            Recording::Status => {
                let status = client
                    .recording()
                    .status()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Recording Status:");
                println!("  Active: {}", status.active);
                if status.active {
                    println!("  Paused: {}", status.paused);
                    println!("  Timecode: {}", status.timecode);
                    println!("  Bytes: {}", status.bytes);
                }
            }
            Recording::StatusActive => {
                let status = client
                    .recording()
                    .status()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                if status.active && !status.paused {
                    println!("Recording is active and running");
                } else if !status.active {
                    return Err(ObsCmdError::RecordingNotActive);
                } else {
                    return Err(ObsCmdError::RecordingPaused);
                }
            }
            Recording::Pause => {
                println!("Pausing recording...");
                client
                    .recording()
                    .pause()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Recording paused successfully");
            }
            Recording::Resume => {
                println!("Resuming recording...");
                client
                    .recording()
                    .resume()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Recording resumed successfully");
            }
            Recording::TogglePause => {
                println!("Toggling recording pause...");
                client
                    .recording()
                    .toggle_pause()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                println!("Recording pause toggled successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            Recording::Start => "Start recording",
            Recording::Stop => "Stop recording",
            Recording::Toggle => "Toggle recording",
            Recording::Status => "Get recording status",
            Recording::StatusActive => "Check if recording is active",
            Recording::Pause => "Pause recording",
            Recording::Resume => "Resume recording",
            Recording::TogglePause => "Toggle recording pause",
        }
    }
}

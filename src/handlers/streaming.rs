use crate::cli::Streaming;
use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for streaming-related commands
pub struct StreamingHandler {
    pub action: Streaming,
}

#[async_trait::async_trait]
impl CommandHandler for StreamingHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            Streaming::Start => {
                println!("Starting stream...");
                client
                    .streaming()
                    .start()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Stream started successfully");
            }
            Streaming::Stop => {
                println!("Stopping stream...");
                client
                    .streaming()
                    .stop()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Stream stopped successfully");
            }
            Streaming::Status => {
                let status = client
                    .streaming()
                    .status()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Streaming Status:");
                println!("  Active: {}", status.active);
                if status.active {
                    println!("  Reconnecting: {}", status.reconnecting);
                    println!("  Timecode: {}", status.timecode);
                    println!("  Duration: {}ms", status.duration);
                    println!("  Bytes sent: {}", status.bytes);
                    println!("  Skipped frames: {}", status.skipped_frames);
                    println!("  Total frames: {}", status.total_frames);
                }
            }
            Streaming::Toggle => {
                println!("Toggling stream...");
                client
                    .streaming()
                    .toggle()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Stream toggled successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            Streaming::Start => "Start streaming",
            Streaming::Stop => "Stop streaming",
            Streaming::Status => "Get streaming status",
            Streaming::Toggle => "Toggle streaming",
        }
    }
}

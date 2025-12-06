use crate::cli::VirtualCamera;
use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for virtual camera commands
pub struct VirtualCameraHandler {
    pub action: VirtualCamera,
}

#[async_trait::async_trait]
impl CommandHandler for VirtualCameraHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            VirtualCamera::Start => {
                println!("Starting virtual camera...");
                client
                    .virtual_cam()
                    .start()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Virtual camera started successfully");
            }
            VirtualCamera::Stop => {
                println!("Stopping virtual camera...");
                client
                    .virtual_cam()
                    .stop()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Virtual camera stopped successfully");
            }
            VirtualCamera::Toggle => {
                println!("Toggling virtual camera...");
                client
                    .virtual_cam()
                    .toggle()
                    .await
                    .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
                println!("Virtual camera toggled successfully");
            }
        };
        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            VirtualCamera::Start => "Start virtual camera",
            VirtualCamera::Stop => "Stop virtual camera",
            VirtualCamera::Toggle => "Toggle virtual camera",
        }
    }
}

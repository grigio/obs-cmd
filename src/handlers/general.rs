use crate::error::Result;
use crate::handlers::CommandHandler;
use obws::Client;

// Handler to list hotkeys
pub struct HotkeyLister;

#[async_trait::async_trait]
impl CommandHandler for HotkeyLister {
    async fn execute(&self, client: &Client) -> Result<()> {
        let hotkeys = client.hotkeys().list().await?;
        for hotkey in hotkeys {
            println!("{}", hotkey);
        }
        Ok(())
    }

    fn description(&self) ->  &'static str {
        "Lists All Hotkeys"
    }
}

/// Handler for hotkey trigger commands
pub struct HotkeyHandler {
    pub name: String,
}

#[async_trait::async_trait]
impl CommandHandler for HotkeyHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        println!("Triggering hotkey: {}", self.name);
        client
            .hotkeys()
            .trigger_by_name(&self.name, None)
            .await
            .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
        println!("Hotkey triggered successfully");
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Trigger hotkey"
    }
}

/// Handler for info commands
pub struct InfoHandler;

#[async_trait::async_trait]
impl CommandHandler for InfoHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        let version = client
            .general()
            .version()
            .await
            .map_err(|e| crate::error::ObsCmdError::ConnectionError(e))?;
        println!("OBS Studio Version: {}", version.obs_version);
        println!("OBS WebSocket Version: {}", version.obs_web_socket_version);
        println!("RPC Version: {}", version.rpc_version);
        println!("Platform: {}", version.platform);
        println!(
            "Available Requests: {} total",
            version.available_requests.len()
        );
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Get OBS version information"
    }
}

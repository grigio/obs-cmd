pub mod audio;
pub mod filters;
pub mod general;
pub mod handler_tests;
pub mod media;
pub mod recording;
pub mod replay_buffer;
pub mod scene_collections;
pub mod scene_items;
pub mod scenes;
pub mod sources;
pub mod streaming;
pub mod ui;
pub mod virtual_camera;

use crate::error::{ObsCmdError, Result};
use obws::Client;

/// Common trait for all command handlers
///
/// This trait provides a consistent interface for handling different types of OBS commands,
/// enabling better modularity and extensibility.
#[async_trait::async_trait]
pub trait CommandHandler {
    /// Execute command with given OBS client
    #[allow(clippy::result_large_err)]
    async fn execute(&self, client: &Client) -> Result<()>;

    /// Get a description of what this command does
    fn description(&self) -> &'static str;
}

/// Utility function for validating monitor index
pub fn validate_monitor_index(
    monitor_list: &[obws::responses::ui::Monitor],
    index: u8,
) -> Result<()> {
    if monitor_list.len() <= (index as usize) {
        return Err(ObsCmdError::MonitorNotAvailable {
            index: index as u32,
        });
    }
    Ok(())
}

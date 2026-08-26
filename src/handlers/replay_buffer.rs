use crate::cli::Replay;
use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::Client;
use std::time::Duration;

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
                // Verify replay buffer is active before attempting to save.
                // Fixes #103: previously `save` could succeed even when the
                // buffer was misconfigured or not running, producing replays of
                // unexpected length (e.g. 3-4 mins instead of configured 60s).
                // OBS's replay buffer length is determined solely by OBS
                // settings (Output → Replay Buffer → Maximum Replay Time /
                // Memory Limit). The buffer must be restarted after changing
                // those settings, otherwise the old limits remain in effect.
                let active = client
                    .replay_buffer()
                    .status()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                if !active {
                    return Err(ObsCmdError::ReplayBufferNotActive);
                }

                // Capture previous path to detect that a new file was written.
                let previous_path = client
                    .replay_buffer()
                    .last_replay()
                    .await
                    .ok()
                    .filter(|p| !p.is_empty());

                println!("Saving replay buffer...");
                client
                    .replay_buffer()
                    .save()
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                // OBS processes SaveReplayBuffer asynchronously. Poll
                // GetLastReplayBufferReplay until the path changes or a
                // non-empty path appears. This ensures the file is flushed
                // before we report success and avoids race where VLC reads an
                // incomplete mux (which can appear as wrong duration).
                let mut saved_path: Option<String> = None;
                for _ in 0..10 {
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    if let Ok(path) = client.replay_buffer().last_replay().await {
                        if !path.is_empty() && previous_path.as_ref() != Some(&path) {
                            saved_path = Some(path);
                            break;
                        }
                    }
                }
                // Fallback to whatever OBS reports if polling didn't detect a change
                if saved_path.is_none() {
                    if let Ok(path) = client.replay_buffer().last_replay().await {
                        if !path.is_empty() {
                            saved_path = Some(path);
                        }
                    }
                }

                println!("Replay buffer saved successfully");
                if let Some(path) = saved_path {
                    println!("Saved replay: {}", path);
                }

                // Diagnostic hint for #103. The replay length is controlled by
                // OBS, not by obs-cmd. Variable lengths (e.g. 5:21, 2:16,
                // 3-4 mins when 60s is configured) almost always mean the
                // OBS Replay Buffer settings are stale or the buffer wasn't
                // restarted after changing them.
                println!(
                    "Note: replay length is controlled by OBS Settings → Output → Replay Buffer.\n\
                     If the saved file is longer/shorter than expected (e.g. 3-4 mins vs 60s):\n\
                     - Verify 'Maximum Replay Time' is set correctly\n\
                     - Ensure 'Memory Limit' is large enough for the configured time at your bitrate\n\
                     - Restart the replay buffer after changing settings: `obs-cmd replay stop && obs-cmd replay start`"
                );

                // Best-effort: try to surface current Replay Buffer output settings
                // to help user diagnose mismatch. This is optional and failures are ignored.
                if let Ok(outputs) = client.outputs().list().await {
                    if outputs
                        .iter()
                        .any(|o| o.name == "Replay Buffer" || o.name == "replay_buffer")
                    {
                        // Attempt to fetch settings for diagnostic; ignore errors
                        let settings_res: std::result::Result<
                            serde_json::Value,
                            obws::error::Error,
                        > = client.outputs().settings("Replay Buffer").await;
                        if let Ok(val) = settings_res {
                            // Common keys seen in OBS: RecRBTime (seconds), RecRBSize (MB)
                            if let Some(t) = val
                                .get("RecRBTime")
                                .or_else(|| val.get("max_time_sec"))
                                .or_else(|| val.get("maximum_replay_time"))
                            {
                                println!("OBS Replay Buffer setting - Maximum Replay Time: {}", t);
                            }
                            if let Some(m) = val
                                .get("RecRBSize")
                                .or_else(|| val.get("memory_limit_mb"))
                                .or_else(|| val.get("maximum_memory"))
                            {
                                println!("OBS Replay Buffer setting - Memory Limit: {} MB", m);
                            }
                        }
                    }
                }
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

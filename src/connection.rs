#![allow(clippy::redundant_closure)]

use crate::error::{ObsCmdError, Result};
use obws::Client;
use std::time::Duration;
use tokio::time::timeout;

/// Configuration for OBS WebSocket connection attempts.
///
/// This struct defines how connection attempts should be handled,
/// including timeouts, retry limits, and delays between attempts.
pub struct ConnectionConfig {
    /// Maximum duration to wait for a single connection attempt
    pub timeout_duration: Duration,
    /// Maximum number of connection attempts before giving up
    pub max_retries: u32,
    /// Duration to wait between retry attempts
    pub retry_delay: Duration,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            // 10 second timeout for each connection attempt
            timeout_duration: Duration::from_secs(10),
            // Try up to 3 times before giving up
            max_retries: 3,
            // Wait 2 seconds between retry attempts
            retry_delay: Duration::from_secs(2),
        }
    }
}

/// Establishes a WebSocket connection to OBS with retry logic.
///
/// This function attempts to connect to an OBS WebSocket server with the
/// provided configuration. It will retry connection attempts according to
/// the specified config and provide detailed error feedback.
///
/// # Arguments
///
/// * `hostname` - The OBS WebSocket server hostname or IP address
/// * `port` - The port number where OBS WebSocket is listening
/// * `password` - Optional password for OBS WebSocket authentication
/// * `config` - Connection configuration including timeouts and retry settings
///
/// # Returns
///
/// Returns a connected `Client` instance on success, or an error if all
/// connection attempts fail.
///
/// # Examples
///
/// ```rust
/// let config = ConnectionConfig::default();
/// let client = connect_with_retry(
///     "localhost".to_string(),
///     4455,
///     Some("secret".to_string()),
///     config
/// ).await?;
/// ```
pub async fn connect_with_retry(
    hostname: String,
    port: u16,
    password: Option<String>,
    config: ConnectionConfig,
) -> Result<Client> {
    let mut last_error = None;

    for attempt in 1..=config.max_retries {
        let connect_result = timeout(
            config.timeout_duration,
            Client::connect(hostname.clone(), port, password.clone()),
        )
        .await;

        match connect_result {
            Ok(Ok(client)) => {
                if attempt > 1 {
                    eprintln!("Connected to OBS after {} attempts", attempt);
                }
                return Ok(client);
            }
            Ok(Err(e)) => {
                last_error = Some(ObsCmdError::ConnectionError(e));
            }
            Err(_) => {
                last_error = Some(ObsCmdError::ConnectionTimeout {
                    timeout: config.timeout_duration.as_secs(),
                });
            }
        }

        if attempt < config.max_retries {
            eprintln!(
                "Connection attempt {} failed, retrying in {} seconds...",
                attempt,
                config.retry_delay.as_secs()
            );
            tokio::time::sleep(config.retry_delay).await;
        }
    }

    Err(
        last_error.unwrap_or_else(|| ObsCmdError::AllConnectionAttemptsFailed {
            attempts: config.max_retries,
        }),
    )
}

/// Checks the health of an existing OBS WebSocket connection.
///
/// This function verifies that the connection to OBS is still active
/// and responsive by attempting to retrieve the OBS version.
///
/// # Arguments
///
/// * `client` - The OBS WebSocket client to check
///
/// # Returns
///
/// Returns `Ok(())` if the connection is healthy, or an error if the
/// connection is unresponsive or broken.
pub async fn check_connection_health(client: &Client) -> Result<()> {
    timeout(Duration::from_secs(5), client.general().version())
        .await
        .map_err(|_| ObsCmdError::ConnectionTimeout { timeout: 5 })?
        .map_err(|e| ObsCmdError::ConnectionError(e))?;

    Ok(())
}

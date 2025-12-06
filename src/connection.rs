use crate::error::{ObsCmdError, Result};
use obws::Client;
use std::time::Duration;
use tokio::time::timeout;

pub struct ConnectionConfig {
    pub timeout_duration: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_duration: Duration::from_secs(10),
            max_retries: 3,
            retry_delay: Duration::from_secs(2),
        }
    }
}

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

    Err(last_error.unwrap_or_else(|| {
        ObsCmdError::AllConnectionAttemptsFailed {
            attempts: config.max_retries,
        }
    }))
}

pub async fn check_connection_health(client: &Client) -> Result<()> {
    timeout(Duration::from_secs(5), client.general().version())
        .await
        .map_err(|_| ObsCmdError::ConnectionTimeout { timeout: 5 })?
        .map_err(ObsCmdError::ConnectionError)?;

    Ok(())
}

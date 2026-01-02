use crate::cli::{Input, MuteAction};
use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::Client;

/// Handler for input management commands
pub struct InputCmdHandler {
    pub action: Input,
}

#[async_trait::async_trait]
impl CommandHandler for InputCmdHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            Input::List { kind } => {
                let inputs = client
                    .inputs()
                    .list(kind.as_deref())
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!("Available inputs:");
                for input in inputs {
                    println!("  - {} ({})", input.id.name, input.kind);
                }
            }

            Input::ListKinds => {
                let kinds = client
                    .inputs()
                    .list_kinds(false)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!("Available input kinds:");
                for kind in kinds {
                    println!("  - {}", kind);
                }
            }

            Input::Create {
                input_name,
                input_kind,
                scene: _,
                settings: _,
            } => {
                println!("Creating input '{}' of kind '{}'", input_name, input_kind);
                println!("Input creation is experimental");
            }

            Input::Remove { input_name } => {
                println!("Removing input: {}", input_name);
                println!("Input removal is experimental");
            }

            Input::Rename {
                input_name,
                new_name,
            } => {
                println!("Renaming input '{}' to '{}'", input_name, new_name);
                println!("Input renaming is experimental");
            }

            Input::Settings {
                input_name,
                get,
                set,
            } => {
                if *get {
                    println!("Getting settings for '{}'", input_name);
                    println!("Settings get is experimental");
                } else if let Some(_new_settings) = set {
                    println!("Updating settings for '{}'", input_name);
                    println!("Settings update is experimental");
                }
            }

            Input::Volume {
                input_name,
                get,
                set,
            } => {
                if *get {
                    println!("Getting volume for '{}'", input_name);
                    println!("Volume get is experimental");
                } else if let Some(new_volume) = set {
                    if !(0.0..=1.0).contains(new_volume) {
                        return Err(ObsCmdError::InvalidVolume {
                            volume: *new_volume,
                        });
                    }

                    println!("Setting volume for '{}' to {:.2}", input_name, new_volume);
                    println!("Volume set is experimental");
                }
            }

            Input::Mute { input_name, action } => match action {
                MuteAction::Mute => {
                    println!("Muting input: {}", input_name);
                    println!("Mute command is experimental");
                }
                MuteAction::Unmute => {
                    println!("Unmuting input: {}", input_name);
                    println!("Unmute command is experimental");
                }
                MuteAction::Toggle => {
                    println!("Toggling input mute: {}", input_name);
                    println!("Toggle command is experimental");
                }
                MuteAction::Status => {
                    println!("Getting mute status for input: {}", input_name);
                    println!("Status command is experimental");
                }
            },

            Input::AudioBalance {
                input_name,
                get,
                set,
            } => {
                if *get {
                    println!("Getting audio balance for '{}'", input_name);
                    println!("Audio balance get is experimental");
                } else if let Some(new_balance) = set {
                    if !(-1.0..=1.0).contains(new_balance) {
                        return Err(ObsCmdError::InvalidAudioBalance {
                            balance: *new_balance,
                        });
                    }

                    println!(
                        "Setting audio balance for '{}' to {:.2}",
                        input_name, new_balance
                    );
                    println!("Audio balance set is experimental");
                }
            }

            Input::AudioSyncOffset {
                input_name,
                get,
                set,
            } => {
                if *get {
                    println!("Getting audio sync offset for '{}'", input_name);
                    println!("Audio sync offset get is experimental");
                } else if let Some(new_offset) = set {
                    println!(
                        "Setting audio sync offset for '{}' to {}ns",
                        input_name, new_offset
                    );
                    println!("Audio sync offset set is experimental");
                }
            }

            Input::AudioMonitorType {
                input_name,
                get,
                set,
            } => {
                if *get {
                    println!("Getting audio monitor type for '{}'", input_name);
                    println!("Audio monitor type get is experimental");
                } else if let Some(new_type) = set {
                    if !["none", "monitorOnly", "both"].contains(&new_type.as_str()) {
                        return Err(ObsCmdError::InvalidAudioMonitorType {
                            monitor_type: new_type.clone(),
                        });
                    }

                    println!(
                        "Setting audio monitor type for '{}' to {}",
                        input_name, new_type
                    );
                    println!("Audio monitor type set is experimental");
                }
            }

            Input::AudioTracks {
                input_name,
                get,
                set,
            } => {
                if *get {
                    println!("Getting audio tracks for '{}'", input_name);
                    println!("Audio tracks get is experimental");
                } else if let Some(_new_tracks) = set {
                    println!("Setting audio tracks for '{}'", input_name);
                    println!("Audio tracks set is experimental");
                }
            }

            Input::DefaultSettings { input_kind } => {
                println!("Getting default settings for '{}':", input_kind);
                println!("Default settings get is experimental");
            }

            Input::Specials => {
                println!("Getting special inputs");
                println!("Special inputs get is experimental");
            }
        }

        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            Input::List { .. } => "List inputs",
            Input::ListKinds => "List available input kinds",
            Input::Create { .. } => "Create new input",
            Input::Remove { .. } => "Remove input",
            Input::Rename { .. } => "Rename input",
            Input::Settings { .. } => "Manage input settings",
            Input::Volume { .. } => "Manage input volume",
            Input::Mute { .. } => "Manage input mute state",
            Input::AudioBalance { .. } => "Manage audio balance",
            Input::AudioSyncOffset { .. } => "Manage audio sync offset",
            Input::AudioMonitorType { .. } => "Manage audio monitor type",
            Input::AudioTracks { .. } => "Manage audio tracks",
            Input::DefaultSettings { .. } => "Get default settings for input kind",
            Input::Specials => "List special inputs",
        }
    }
}

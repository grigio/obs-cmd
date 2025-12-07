use crate::cli::SceneItem;
use crate::error::{ObsCmdError, Result};
use crate::handlers::CommandHandler;
use obws::common::BlendMode;
use obws::requests::scene_items::{
    CreateSceneItem, Crop, Duplicate, Id as IdItem, Position, Scale, SceneItemTransform,
    SetBlendMode, SetEnabled as SetEnabledItem, SetIndex, SetLocked, SetTransform,
};
use obws::requests::scenes::SceneId;
use obws::requests::sources::SourceId;
use obws::Client;

/// Handler for scene item commands
pub struct SceneItemHandler {
    pub action: SceneItem,
}

#[async_trait::async_trait]
impl CommandHandler for SceneItemHandler {
    async fn execute(&self, client: &Client) -> Result<()> {
        match &self.action {
            SceneItem::List { scene } => {
                let items = client
                    .scene_items()
                    .list(SceneId::Name(scene))
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!("Scene items in '{}':", scene);
                for item in items {
                    println!(
                        "  - ID: {}, Source: '{}', Index: {}",
                        item.id, item.source_name, item.index
                    );
                }
            }
            SceneItem::Create {
                scene,
                source,
                enabled,
            } => {
                let item_id = client
                    .scene_items()
                    .create(CreateSceneItem {
                        scene: SceneId::Name(scene),
                        source: SourceId::Name(source),
                        enabled: *enabled,
                    })
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Created scene item '{}' in scene '{}' with ID: {}",
                    source, scene, item_id
                );
            }
            SceneItem::Remove { scene, source } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                client
                    .scene_items()
                    .remove(SceneId::Name(target_scene), item_id)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Removed scene item '{}' from {}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    }
                );
            }
            SceneItem::Duplicate { scene, source } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                let new_item_id = client
                    .scene_items()
                    .duplicate(Duplicate {
                        scene: SceneId::Name(target_scene),
                        item_id,
                        destination: None, // Duplicate to same scene/group
                    })
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Duplicated scene item '{}' in {} with new ID: {}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    },
                    new_item_id
                );
            }
            SceneItem::Enable { scene, source } => {
                set_scene_item_enabled(client, scene, source, true).await?;
                println!("Enabled scene item '{}' in scene '{}'", source, scene);
            }
            SceneItem::Disable { scene, source } => {
                set_scene_item_enabled(client, scene, source, false).await?;
                println!("Disabled scene item '{}' in scene '{}'", source, scene);
            }
            SceneItem::Toggle { scene, source } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                let current_state = client
                    .scene_items()
                    .enabled(SceneId::Name(target_scene), item_id)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;
                let new_state = !current_state;

                set_scene_item_enabled(client, target_scene, source, new_state).await?;
                println!(
                    "Scene item '{}' in {}: {}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    },
                    if new_state { "enabled" } else { "disabled" }
                );
            }
            SceneItem::Lock { scene, source } => {
                set_scene_item_locked(client, scene, source, true).await?;
                println!("Locked scene item '{}' in scene '{}'", source, scene);
            }
            SceneItem::Unlock { scene, source } => {
                set_scene_item_locked(client, scene, source, false).await?;
                println!("Unlocked scene item '{}' in scene '{}'", source, scene);
            }
            SceneItem::GetTransform { scene, source } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                let transform = client
                    .scene_items()
                    .transform(SceneId::Name(target_scene), item_id)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Transform for scene item '{}' in {}:",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    }
                );
                println!(
                    "  Position: X: {}, Y: {}",
                    transform.position_x, transform.position_y
                );
                println!(
                    "  Scale: X: {}, Y: {}",
                    transform.scale_x, transform.scale_y
                );
                println!("  Rotation: {} degrees", transform.rotation);
                println!(
                    "  Crop: Left: {}, Right: {}, Top: {}, Bottom: {}",
                    transform.crop_left,
                    transform.crop_right,
                    transform.crop_top,
                    transform.crop_bottom
                );
            }
            SceneItem::SetTransform {
                scene,
                source,
                position_x,
                position_y,
                scale_x,
                scale_y,
                rotation,
                crop_left,
                crop_right,
                crop_top,
                crop_bottom,
            } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;

                // Get current transform to use as base
                let _current_transform = client
                    .scene_items()
                    .transform(SceneId::Name(target_scene), item_id)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                let transform = SceneItemTransform {
                    position: if position_x.is_some() || position_y.is_some() {
                        Some(Position {
                            x: position_x.map(|x| x as f32),
                            y: position_y.map(|y| y as f32),
                        })
                    } else {
                        None
                    },
                    scale: if scale_x.is_some() || scale_y.is_some() {
                        Some(Scale {
                            x: scale_x.map(|x| x as f32),
                            y: scale_y.map(|y| y as f32),
                        })
                    } else {
                        None
                    },
                    rotation: rotation.map(|r| r as f32),
                    crop: if crop_left.is_some()
                        || crop_right.is_some()
                        || crop_top.is_some()
                        || crop_bottom.is_some()
                    {
                        Some(Crop {
                            left: *crop_left,
                            right: *crop_right,
                            top: *crop_top,
                            bottom: *crop_bottom,
                        })
                    } else {
                        None
                    },
                    alignment: None, // Keep current alignment
                    bounds: None,    // Keep current bounds
                };

                client
                    .scene_items()
                    .set_transform(SetTransform {
                        scene: SceneId::Name(target_scene),
                        item_id,
                        transform,
                    })
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Updated transform for scene item '{}' in {}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    }
                );
            }
            SceneItem::GetIndex { scene, source } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                let index = client
                    .scene_items()
                    .index(SceneId::Name(target_scene), item_id)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Scene item '{}' in {} has index: {}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    },
                    index
                );
            }
            SceneItem::SetIndex {
                scene,
                source,
                index,
            } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                client
                    .scene_items()
                    .set_index(SetIndex {
                        scene: SceneId::Name(target_scene),
                        item_id,
                        index: *index,
                    })
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Set scene item '{}' in {} to index: {}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    },
                    index
                );
            }
            SceneItem::GetBlendMode { scene, source } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                let blend_mode = client
                    .scene_items()
                    .blend_mode(SceneId::Name(target_scene), item_id)
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Scene item '{}' in {} has blend mode: {:?}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    },
                    blend_mode
                );
            }
            SceneItem::SetBlendMode {
                scene,
                source,
                blend_mode,
            } => {
                // Check if source path contains a group separator "/"
                let target_scene = if let Some(slash_pos) = source.find('/') {
                    &source[..slash_pos]
                } else {
                    scene
                };

                let item_id = get_scene_item_id(client, target_scene, source).await?;
                let parsed_blend_mode = parse_blend_mode(blend_mode)?;

                client
                    .scene_items()
                    .set_blend_mode(SetBlendMode {
                        scene: SceneId::Name(target_scene),
                        item_id,
                        mode: parsed_blend_mode,
                    })
                    .await
                    .map_err(|e| ObsCmdError::ConnectionError(e))?;

                println!(
                    "Set scene item '{}' in {} blend mode to: {:?}",
                    source,
                    if target_scene == scene {
                        format!("scene '{}'", scene)
                    } else {
                        format!("group '{}' in scene '{}'", target_scene, scene)
                    },
                    parsed_blend_mode
                );
            }
        }

        Ok(())
    }

    fn description(&self) -> &'static str {
        match &self.action {
            SceneItem::List { .. } => "List scene items in a scene",
            SceneItem::Create { .. } => "Create a new scene item",
            SceneItem::Remove { .. } => "Remove a scene item",
            SceneItem::Duplicate { .. } => "Duplicate a scene item",
            SceneItem::Enable { .. } => "Enable a scene item",
            SceneItem::Disable { .. } => "Disable a scene item",
            SceneItem::Toggle { .. } => "Toggle a scene item",
            SceneItem::Lock { .. } => "Lock a scene item",
            SceneItem::Unlock { .. } => "Unlock a scene item",
            SceneItem::GetTransform { .. } => "Get scene item transform",
            SceneItem::SetTransform { .. } => "Set scene item transform",
            SceneItem::GetIndex { .. } => "Get scene item index",
            SceneItem::SetIndex { .. } => "Set scene item index",
            SceneItem::GetBlendMode { .. } => "Get scene item blend mode",
            SceneItem::SetBlendMode { .. } => "Set scene item blend mode",
        }
    }
}

/// Helper function to get scene item ID by scene and source name
/// Supports both regular scenes and groups (for nested sources)
/// For nested sources in groups, source can be specified as "Group/SourceName"
async fn get_scene_item_id(client: &Client, scene: &str, source: &str) -> Result<i64> {
    // Check if source path contains a group separator "/"
    if let Some(slash_pos) = source.find('/') {
        // Split into group name and source name
        let group_name = &source[..slash_pos];
        let nested_source_name = &source[slash_pos + 1..];

        // Search within the group
        client
            .scene_items()
            .id(IdItem {
                scene: SceneId::Name(group_name),
                source: nested_source_name,
                search_offset: Some(0),
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))
    } else {
        // Direct search in the scene
        client
            .scene_items()
            .id(IdItem {
                scene: SceneId::Name(scene),
                source,
                search_offset: Some(0),
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))
    }
}

/// Helper function to set scene item enabled state
async fn set_scene_item_enabled(
    client: &Client,
    scene: &str,
    source: &str,
    enabled: bool,
) -> Result<()> {
    // Check if source path contains a group separator "/"
    if let Some(slash_pos) = source.find('/') {
        // Split into group name and source name
        let group_name = &source[..slash_pos];
        let nested_source_name = &source[slash_pos + 1..];

        let item_id = get_scene_item_id(client, group_name, nested_source_name).await?;
        client
            .scene_items()
            .set_enabled(SetEnabledItem {
                scene: SceneId::Name(group_name),
                item_id,
                enabled,
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))
    } else {
        let item_id = get_scene_item_id(client, scene, source).await?;
        client
            .scene_items()
            .set_enabled(SetEnabledItem {
                scene: SceneId::Name(scene),
                item_id,
                enabled,
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))
    }
}

/// Helper function to set scene item locked state
async fn set_scene_item_locked(
    client: &Client,
    scene: &str,
    source: &str,
    locked: bool,
) -> Result<()> {
    // Check if source path contains a group separator "/"
    if let Some(slash_pos) = source.find('/') {
        // Split into group name and source name
        let group_name = &source[..slash_pos];
        let nested_source_name = &source[slash_pos + 1..];

        let item_id = get_scene_item_id(client, group_name, nested_source_name).await?;
        client
            .scene_items()
            .set_locked(SetLocked {
                scene: SceneId::Name(group_name),
                item_id,
                locked,
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))
    } else {
        let item_id = get_scene_item_id(client, scene, source).await?;
        client
            .scene_items()
            .set_locked(SetLocked {
                scene: SceneId::Name(scene),
                item_id,
                locked,
            })
            .await
            .map_err(|e| ObsCmdError::ConnectionError(e))
    }
}

/// Parse blend mode string to BlendMode enum
fn parse_blend_mode(blend_mode: &str) -> Result<BlendMode> {
    match blend_mode.to_lowercase().as_str() {
        "normal" => Ok(BlendMode::Normal),
        "additive" => Ok(BlendMode::Additive),
        "subtract" => Ok(BlendMode::Subtract),
        "screen" => Ok(BlendMode::Screen),
        "multiply" => Ok(BlendMode::Multiply),
        "lighten" => Ok(BlendMode::Lighten),
        "darken" => Ok(BlendMode::Darken),
        _ => Err(ObsCmdError::InvalidBlendMode {
            blend_mode: blend_mode.to_string(),
        }),
    }
}

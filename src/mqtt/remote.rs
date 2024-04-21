use std::collections::HashMap;

use crate::db::data::DeviceType;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RemoteAction {
    pub action: RemoteEvent,
    pub link_quality: Option<i32>,
}
impl RemoteAction {
    pub fn from_type(payload: &str, device_type: DeviceType) -> Result<Self> {
        let parsed = serde_json::from_str::<HashMap<String, Value>>(payload)?;
        let link_quality = parsed
            .get("linkquality")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let action = parsed
            .get("action")
            .ok_or(anyhow::Error::msg("missing action field"))?
            .clone();

        let action = match device_type {
            DeviceType::TradfriRemoteN2 => RemoteEvent::TradfriStyrbarAction(
                serde_json::from_value::<TradfriStyrbarAction>(action)?,
            ),
            x => RemoteEvent::UnknownAction(format!("{x:?}")),
        };

        Ok(RemoteAction {
            link_quality,
            action,
        })
    }
}

#[derive(Debug, Clone)]
pub enum RemoteEvent {
    UnknownAction(String),
    TradfriStyrbarAction(TradfriStyrbarAction),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum TradfriStyrbarAction {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "brightness_move_up")]
    BrightnessMoveUp,
    #[serde(rename = "brightness_stop")]
    BrightnessStop,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "brightness_move_down")]
    BrightnessMoveDown,
    #[serde(rename = "arrow_left_click")]
    ArrowLeftClick,
    #[serde(rename = "arrow_left_hold")]
    ArrowLeftHold,
    #[serde(rename = "arrow_left_release")]
    ArrowLeftRelease,
    #[serde(rename = "arrow_right_click")]
    ArrowRightClick,
    #[serde(rename = "arrow_right_hold")]
    ArrowRightHold,
    #[serde(rename = "arrow_right_release")]
    ArrowRightRelease,
}

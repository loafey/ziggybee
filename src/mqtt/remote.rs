use std::collections::HashMap;

use crate::db::DeviceType;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl RemoteEvent {
    pub fn from_type(payload: &str, device_type: DeviceType) -> Result<Self> {
        let parsed = serde_json::from_str::<HashMap<String, Value>>(payload)?;

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

        Ok(action)
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct RemoteAction {
    pub action: TradfriStyrbarAction,
    pub link_quality: Option<i32>,
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

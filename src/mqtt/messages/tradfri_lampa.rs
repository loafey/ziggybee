use serde::{Deserialize, Serialize};

use super::Vec2;

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum TradfriState {
    #[default]
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off,
}
impl From<bool> for TradfriState {
    fn from(value: bool) -> Self {
        match value {
            true => TradfriState::On,
            false => TradfriState::Off,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TradfriLampaMsg {
    pub brightness: Option<u8>,
    pub color: Option<Vec2>,
    pub color_temp: Option<u8>,
    pub state: Option<TradfriState>,
}

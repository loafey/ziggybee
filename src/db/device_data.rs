use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    TradfriBulb,
    TradfriRemoteN2,
    UnknownDevice(String),
}
impl DeviceType {
    pub fn should_subscribe(&self) -> bool {
        matches!(self, Self::TradfriRemoteN2)
    }
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::UnknownDevice("Unknown device".to_string())
    }
}
impl From<String> for DeviceType {
    fn from(value: String) -> Self {
        match &value[..] {
            "TRADFRI bulb E27 CWS 806lm" => DeviceType::TradfriBulb,
            "Remote Control N2" => DeviceType::TradfriRemoteN2,
            _ => DeviceType::UnknownDevice(value),
        }
    }
}

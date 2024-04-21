use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Setup {
    pub setups: Vec<Endpoint>,
    pub unsorted: Vec<Endpoint>,
}
impl Setup {
    pub fn contains(&self, id: &str) -> bool {
        self.setups.iter().any(|c| c.contains(id)) || self.unsorted.iter().any(|c| c.contains(id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Endpoint {
    Device {
        uri: String,
        name: String,
        r#type: DeviceType,
    },
    Endpoint {
        name: String,
        #[serde(default)]
        children: Vec<Endpoint>,
    },
}
impl Endpoint {
    pub fn contains(&self, id: &str) -> bool {
        match self {
            Endpoint::Device { uri, .. } => uri == id,
            Endpoint::Endpoint { children, .. } => children.iter().any(|a| a.contains(id)),
        }
    }
}

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

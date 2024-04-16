use paho_mqtt::AsyncClient;
use serde::{Deserialize, Serialize};

pub mod tradfri_bulb;
pub mod tradfri_remote_control_n2;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum OnState {
    #[default]
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "TOGGLE")]
    Toggle,
}
impl From<bool> for OnState {
    fn from(value: bool) -> Self {
        match value {
            true => OnState::On,
            false => OnState::Off,
        }
    }
}

pub trait Device<State> {
    async fn publish(&self, state: State) -> anyhow::Result<()>;
    fn subscribe(&self, cli: &AsyncClient);
}

use std::{collections::HashMap, sync::LazyLock};

use super::devices::{tradfri_remote_control_n2::TradfriStyrbar, Device};
use paho_mqtt::AsyncClient;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub static ENDPOINTS: LazyLock<RwLock<Endpoint>> = LazyLock::new(|| RwLock::new(load_endpoints()));

pub fn subscribe(cli: &AsyncClient) {
    TradfriStyrbar {
        data: "zigbee2mqtt/0x5cc7c1fffe8b7a9d/#",
    }
    .subscribe(cli);
}

fn load_endpoints() -> Endpoint {
    let str = std::fs::read_to_string("devices.json").unwrap();
    serde_json::from_str::<Endpoint>(&str).unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Endpoint {
    Device(DeviceEndpoint),
    Room(HashMap<String, Endpoint>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceEndpoint {
    url: String,
    #[serde(rename = "type")]
    r#type: DeviceType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeviceType {
    HostDevice,
    TradfriBulb,
}

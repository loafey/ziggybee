use super::raw_data::get_db;
use anyhow::Result;
use log::error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    sync::LazyLock,
};
use tokio::sync::{RwLock, RwLockReadGuard};

// Global
static DEVICES: LazyLock<RwLock<HashMap<String, Device>>> = LazyLock::new(|| {
    let res = futures::executor::block_on(load_device_data()).unwrap();
    RwLock::new(res)
});

// Types
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Device {
    pub name: String,
    pub r#type: DeviceType,
}

pub async fn get_device(device: &str) -> Option<Device> {
    DEVICES.read().await.get(device).cloned()
}

// Functions
pub async fn get_device_data() -> RwLockReadGuard<'static, HashMap<String, Device>> {
    DEVICES.read().await
}

pub async fn reload_device_data() {
    match load_device_data().await {
        Ok(o) => *DEVICES.write().await = o,
        Err(e) => error!("failed reloading device data: {e}"),
    }
}

async fn load_device_data() -> Result<HashMap<String, Device>> {
    let mut setup = match read_to_string("data/devices.json")
        .map(|s| serde_json::from_str::<HashMap<String, Device>>(&s))
    {
        Ok(f) => match f {
            Ok(f) => f,
            Err(e) => {
                error!("failed parsing setup: {e}");
                HashMap::default()
            }
        },
        Err(e) => {
            error!("failed loading setup: {e}");
            HashMap::default()
        }
    };

    let db = get_db().await;
    for (k, d) in db.iter() {
        if !setup.contains_key(k) {
            setup.insert(
                k.clone(),
                Device {
                    name: d.model_id.clone().unwrap_or("Unknown device".to_string()),
                    r#type: DeviceType::from(
                        d.model_id.clone().unwrap_or("Unknown device".to_string()),
                    ),
                },
            );
        }
    }

    let mut f = File::create("data/devices.json")?;
    f.write_all(&serde_json::to_vec(&setup).unwrap())?;

    Ok(setup)
}

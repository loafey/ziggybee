use super::data::DeviceType;
use crate::mqtt::subscribe;
use anyhow::Result;
use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, sync::LazyLock};
use tokio::sync::{RwLock, RwLockReadGuard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "ieeeAddr")]
    pub ieee_addr: String,
    #[serde(rename = "manufName")]
    pub manufacturer_name: Option<String>,
    #[serde(rename = "modelId")]
    pub model_id: Option<String>,
    #[serde(rename = "_____non_existent____")]
    pub device_type: Option<DeviceType>,
}

pub type DB = HashMap<String, Device>;

pub async fn get_device_info(device: &str) -> Option<Device> {
    get_db().await.get(device).cloned()
}

static DB: LazyLock<RwLock<DB>> = LazyLock::new(|| {
    let res = futures::executor::block_on(load_db()).unwrap();

    for (k, val) in res.iter() {
        if let Some(true) = val.device_type.as_ref().map(|v| v.should_subscribe()) {
            subscribe(k);
        }
    }

    RwLock::new(res)
});

pub async fn get_db() -> RwLockReadGuard<'static, DB> {
    DB.read().await
}

pub async fn reload_db() {
    match load_db().await {
        Err(err) => error!("failed to reload DB: {}", err),
        Ok(db) => *DB.write().await = db,
    }
}
pub async fn load_db() -> Result<DB> {
    let db = read_to_string("docker/zigbee2mqtt-data/database.db")?;
    let db = db
        .lines()
        .map(serde_json::from_str::<Device>)
        .map(|r| {
            r.map(|mut d| {
                d.device_type = Some(DeviceType::from(
                    d.model_id.clone().unwrap_or("Unknown device".to_string()),
                ));
                (d.ieee_addr.clone(), d)
            })
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(db)
}

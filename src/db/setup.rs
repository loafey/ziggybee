use super::{data::DeviceType, raw_data::reload_db};
use crate::{db::raw_data::get_db, mqtt::subscribe};
use log::error;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, File},
    io::Write,
    sync::LazyLock,
    time::SystemTime,
};
use tokio::sync::{RwLock, RwLockReadGuard};

// Globals
static LAST_ACCESS: LazyLock<RwLock<SystemTime>> = LazyLock::new(|| RwLock::new(SystemTime::now()));
static SETUP: LazyLock<RwLock<Setup>> = LazyLock::new(|| {
    let res = futures::executor::block_on(load_setup());
    RwLock::new(res)
});

// Setup conf
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

// Endpoint tree
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

// Funcs
pub async fn get_setup() -> RwLockReadGuard<'static, Setup> {
    let time = *LAST_ACCESS.read().await;
    if let Ok(time) = SystemTime::now().duration_since(time) {
        if time.as_secs_f64() > 10.0 {
            reload_db().await;
            *SETUP.write().await = load_setup().await;
        }
    }

    SETUP.read().await
}

async fn load_setup() -> Setup {
    *LAST_ACCESS.write().await = SystemTime::now();
    let mut setup =
        match read_to_string("data/setup.json").map(|s| serde_json::from_str::<Setup>(&s)) {
            Ok(f) => match f {
                Ok(f) => f,
                Err(e) => {
                    error!("failed parsing setup: {e}");
                    Setup::default()
                }
            },
            Err(e) => {
                error!("failed loading setup: {e}");
                Setup::default()
            }
        };

    let db = get_db().await;
    for (k, d) in db.iter() {
        if !setup.contains(k) {
            if let Some(true) = d.device_type.as_ref().map(|d| d.should_subscribe()) {
                subscribe(k);
            }
            setup.unsorted.push(Endpoint::Device {
                uri: k.clone(),
                name: d.model_id.clone().unwrap_or("Unknown device".to_string()),
                r#type: DeviceType::from(
                    d.model_id.clone().unwrap_or("Unknown device".to_string()),
                ),
            })
        }
    }

    let mut f = File::create("data/setup.json").unwrap();
    f.write_all(&serde_json::to_vec(&setup).unwrap()).unwrap();

    setup
}

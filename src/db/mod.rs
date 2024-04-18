use self::data::{DeviceType, Endpoint, Setup};
use anyhow::Result;
use data::Device;
use log::{debug, error};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    sync::LazyLock,
    time::SystemTime,
};
use tokio::sync::{RwLock, RwLockReadGuard};

pub mod data;

pub type DB = HashMap<String, Device>;

static DB: LazyLock<RwLock<DB>> = LazyLock::new(|| {
    let res = futures::executor::block_on(load_db());
    RwLock::new(res.unwrap())
});
static LAST_ACCESS: LazyLock<RwLock<SystemTime>> = LazyLock::new(|| RwLock::new(SystemTime::now()));
static SETUP: LazyLock<RwLock<Setup>> = LazyLock::new(|| {
    let res = futures::executor::block_on(load_setup());
    RwLock::new(res)
});

pub async fn init_db() {
    debug!("getting db");
    let _ = get_db().await;
    debug!("getting setup");
    let _ = get_setup().await;
    debug!("done loading");
}

async fn get_db() -> RwLockReadGuard<'static, DB> {
    let time = *LAST_ACCESS.read().await;
    if let Ok(time) = SystemTime::now().duration_since(time) {
        if time.as_secs_f64() > 10.0 {
            if let Err(err) = load_db().await {
                error!("failed to reload DB: {}", err)
            }
        }
    }

    DB.read().await
}

async fn load_db() -> Result<DB> {
    *LAST_ACCESS.write().await = SystemTime::now();
    let db = read_to_string("docker/zigbee2mqtt-data/database.db")?;
    let db = db
        .lines()
        .map(serde_json::from_str::<Device>)
        .map(|r| r.map(|d| (d.ieee_addr.clone(), d)))
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(db)
}

pub async fn get_setup() -> RwLockReadGuard<'static, Setup> {
    SETUP.read().await
}

async fn load_setup() -> Setup {
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
            setup.unsorted.push(Endpoint::Device {
                uri: k.clone(),
                name: d.model_id.clone().unwrap_or("Unknown device".to_string()),
                r#type: DeviceType::from(d.model_id.clone().unwrap_or_default()),
            })
        }
    }

    let mut f = File::create("data/setup.json").unwrap();
    f.write_all(&serde_json::to_vec(&setup).unwrap()).unwrap();

    setup
}
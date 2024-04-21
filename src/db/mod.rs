mod device_data;
mod raw_data;
mod setup_tree;
pub use device_data::{get_device, Device, DeviceType, Payload};
use log::info;
use raw_data::get_db;
pub use setup_tree::{get_setup_tree, Endpoint, SetupTree};

use crate::db::device_data::get_device_data;

pub async fn init_db() {
    info!("Getting db");
    let _ = get_db().await;
    info!("Getting setup");
    let _ = get_setup_tree().await;
    info!("Getting device data");
    let _ = get_device_data().await;
    info!("Done getting DB, device data and setup");
}

pub mod data;
mod device_data;
mod raw_data;
mod setup_tree;
use log::info;
use raw_data::get_db;
pub use raw_data::get_device_info;
pub use setup_tree::{get_setup_tree, Endpoint, SetupTree};

pub async fn init_db() {
    info!("Getting db");
    let _ = get_db().await;
    info!("Getting setup");
    let _ = get_setup_tree().await;
    info!("Done getting DB and setup");
}

use raw_data::get_db;
pub mod data;
mod raw_data;
mod setup;
use log::info;
pub use raw_data::get_device_info;
pub use setup::{get_setup, Endpoint, Setup};

pub async fn init_db() {
    info!("Getting db");
    let _ = get_db().await;
    info!("Getting setup");
    let _ = get_setup().await;
    info!("Done getting DB and setup");
}

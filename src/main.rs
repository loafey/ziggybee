#![feature(lazy_cell)]

use mqtt::{
    devices,
    messages::{TradfriLampaMsg, Vec2},
};

mod mqtt;

#[tokio::main]
async fn main() {
    // Initialize the logger from the environment
    env_logger::init();

    devices::lamps::living_room
        .send(TradfriLampaMsg {
            brightness: Some(255),
            color: Some(Vec2 { x: 1.0, y: 1.0 }),
            ..Default::default()
        })
        .await
        .unwrap();
}

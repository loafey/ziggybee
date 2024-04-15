#![feature(lazy_cell)]

use mqtt::{
    devices,
    messages::{TradfriLampaMsg, Vec2},
};
use rand::Rng;
use std::{thread::sleep, time};

mod mqtt;

fn main() {
    // Initialize the logger from the environment
    env_logger::init();

    let mut rand = rand::thread_rng();
    for i in (0..).step_by(15) {
        devices::lamps::living_room
            .send(TradfriLampaMsg {
                brightness: Some((i % 256) as u8),
                color: Some(Vec2 {
                    x: rand.gen(),
                    y: rand.gen(),
                }),
                ..Default::default()
            })
            .unwrap();
        sleep(time::Duration::from_secs_f32(0.25))
    }
}

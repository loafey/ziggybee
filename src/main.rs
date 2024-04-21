#![feature(lazy_cell)]

extern crate log;

use crate::db::{get_setup, init_db};

mod db;
mod mqtt;
mod sitegen;
mod web;

#[tokio::main]
async fn main() {
    env_logger::init();
    init_db().await;
    let _ = get_setup().await;

    web::setup().await;
}

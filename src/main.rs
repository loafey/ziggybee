#![feature(lazy_cell)]

extern crate log;

use crate::db::{get_setup, init_db};

mod db;
mod mqtt;
mod sitegen;
mod web;

fn init_log() {
    use simplelog::*;
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    init_log();
    init_db().await;
    let _ = get_setup().await;

    web::setup().await;
}

#![feature(lazy_cell)]

mod mqtt;
mod web;

#[tokio::main]
async fn main() {
    env_logger::init();
    web::setup().await;
}

#![feature(lazy_cell)]

use mqtt::endpoints::ENDPOINTS;

mod mqtt;
mod web;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("{:#?}", ENDPOINTS.read().await);
    web::setup().await;
}

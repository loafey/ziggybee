use paho_mqtt::AsyncClient;
use std::{
    env, process,
    sync::{Arc, LazyLock},
};

pub mod endpoints;
pub mod messages;
pub use endpoints::devices;

const QOS: i32 = 1;

static CLI: LazyLock<Arc<AsyncClient>> = LazyLock::new(|| Arc::new(init()));

fn init() -> AsyncClient {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    // Create a client & define connect options
    let cli = paho_mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

    let conn_opts = paho_mqtt::ConnectOptions::new();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts).wait() {
        println!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    cli
}

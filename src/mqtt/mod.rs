use futures::StreamExt as _;
use log::error;
use paho_mqtt::{AsyncClient, AsyncReceiver, Message};
use std::{
    env, process,
    sync::{Arc, LazyLock},
};
use tokio::sync::Mutex;

pub mod remote;

const QOS: i32 = 1;

type Stream = Mutex<AsyncReceiver<Option<Message>>>;

static CLI: LazyLock<Arc<AsyncClient>> = LazyLock::new(|| Arc::new(init()));

pub async fn publish_to_device(device: &str, msg: &str) {
    let topic = paho_mqtt::Topic::new(&CLI, format!("zigbee2mqtt/{device}/set"), QOS);

    if let Err(err) = topic.publish(msg).await {
        error!("failed to publish message: {err}");
    };
}

pub async fn subscribe(topic: &str) {}

fn init() -> AsyncClient {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    // Create a client & define connect options
    let mut cli = paho_mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        error!("Error creating the client: {}", err);
        process::exit(1);
    });

    let conn_opts = paho_mqtt::ConnectOptions::new();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts).wait() {
        error!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    let stream = Mutex::new(cli.get_stream(25));
    subscription_loop(stream);

    // Subscribe to remotes
    // endpoints::subscribe(&cli);

    cli
}

fn subscription_loop(strm: Stream) {
    tokio::task::spawn(async move {
        while let Some(msg_opt) = strm.lock().await.next().await {
            if let Some(msg) = msg_opt {
                println!("unhandled event: {msg:?}");
            } else {
                panic!("lost connection")
            }
        }
    });
}

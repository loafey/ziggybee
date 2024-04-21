use futures::StreamExt as _;
use log::{error, info};
use paho_mqtt::{AsyncClient, AsyncReceiver, Message};
use std::{
    env, process,
    sync::{Arc, LazyLock},
};
use tokio::sync::Mutex;

use crate::{
    db::{get_device, Payload},
    mqtt::remote::RemoteEvent,
};

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

pub fn subscribe(topic: &str) {
    info!("Subscribing to {topic:?}");
    if let Err(e) = CLI.subscribe(format!("zigbee2mqtt/{topic}/#"), QOS).wait() {
        error!("Failed to subscribe to {topic:?}: {e}")
    }
}

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
        info!("Starting subscription loop...");
        while let Some(msg_opt) = strm.lock().await.next().await {
            if let Some(msg) = msg_opt {
                let topic = msg.topic().split_once('/').unwrap().1;
                let payload = msg.payload_str();

                info!("Got message [{payload}] from {topic:?}");

                let Some(info) = get_device(topic).await else {
                    error!("message from unknown device {topic:?}");
                    continue;
                };

                let action = match RemoteEvent::from_type(&payload, info.r#type) {
                    Ok(o) => serde_json::to_string(&o).unwrap(),
                    Err(e) => {
                        error!("invalid payload: {payload:?}, {e}");
                        continue;
                    }
                };

                if let Some(Payload { target, code }) = info.actions.get(&action) {
                    publish_to_device(target, code).await;
                } else {
                    error!(
                        "No action ({}) available for device {topic:?}",
                        serde_json::to_string(&action).unwrap()
                    )
                }
            } else {
                panic!("lost connection")
            }
        }
    });
}

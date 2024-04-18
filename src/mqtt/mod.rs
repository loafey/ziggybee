use paho_mqtt::AsyncClient;
use std::{
    env, process,
    sync::{Arc, LazyLock},
};

pub mod remote;

const QOS: i32 = 1;

//type Stream = Mutex<AsyncReceiver<Option<Message>>>;

static CLI: LazyLock<Arc<AsyncClient>> = LazyLock::new(|| Arc::new(init()));

pub async fn publish_to_device(device: &str, msg: &str) {
    let topic = paho_mqtt::Topic::new(&CLI, format!("zigbee2mqtt/{device}/set"), QOS);

    if let Err(err) = topic.publish(msg).await {
        println!("failed to publish message: {err}");
    };
}

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

    // let stream = Mutex::new(cli.get_stream(25));
    // subscription_loop(stream);

    // Subscribe to remotes
    // endpoints::subscribe(&cli);

    cli
}

// fn subscription_loop(strm: Stream) {
//     tokio::task::spawn(async move {
//         while let Some(msg_opt) = strm.lock().await.next().await {
//             if let Some(msg) = msg_opt {
//                 let msg = serde_json::from_slice::<RemoteAction>(msg.payload()).unwrap();
//                 println!("unhandled event: {msg:?}");
//             } else {
//                 panic!("lost connection")
//             }
//         }
//     });
// }

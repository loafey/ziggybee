use crate::{db, mqtt::publish_to_device, sitegen::get_html};
use axum::{
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

pub async fn setup() {
    let app = Router::new()
        .route("/", get(root))
        .route("/get-setup", get(get_setup))
        .route("/publish-device", post(publish_device));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    Html(get_html().await)
}

async fn get_setup() -> Json<db::SetupTree> {
    let data = db::get_setup_tree().await.clone();
    Json(data)
}

#[derive(Deserialize)]
struct DeviceMessage {
    uri: String,
    body: String,
}
async fn publish_device(Json(msg): Json<DeviceMessage>) -> String {
    publish_to_device(&msg.uri, &msg.body).await;

    "Ok".to_owned()
}

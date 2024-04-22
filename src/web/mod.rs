use std::fs::read_to_string;

use crate::{db, mqtt::publish_to_device};
use axum::{
    body::Body,
    extract::Path,
    http::HeaderValue,
    response::{Html, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

pub async fn setup() {
    let app = Router::new()
        .route("/", get(root))
        .route("/get-setup", get(get_setup))
        .route("/index.css", get(index_css))
        .route("/index.js", get(index_js))
        .route("/publish-device", post(publish_device))
        .route("/device-info/:device", get(get_device_info));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Result<Html<String>, String> {
    Ok(Html(
        read_to_string("src/web/www/index.html").map_err(|e| e.to_string())?,
    ))
}

async fn index_js() -> Result<Response, String> {
    let response = Response::new(Body::from(
        read_to_string("src/web/www/index.js").map_err(|e| e.to_string())?,
    ));

    Ok(response)
}

async fn index_css() -> Result<Response, String> {
    let mut response = Response::new(Body::from(
        read_to_string("src/web/www/index.css").map_err(|e| e.to_string())?,
    ));

    response
        .headers_mut()
        .insert("content-type", HeaderValue::from_static("text/css"));

    Ok(response)
}

async fn get_setup() -> Json<db::SetupTree> {
    let data = db::get_setup_tree().await.clone();
    Json(data)
}

async fn get_device_info(Path(device): Path<String>) -> Result<Json<db::Device>, &'static str> {
    db::get_device(&device)
        .await
        .map(Json)
        .ok_or("device not found")
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

use crate::mqtt::devices::{
    tradfri_bulb::{TradfriBulb, TradfriBulbState},
    Device, OnState, Vec2,
};
use axum::{
    response::{Html, Redirect},
    routing::{get, post},
    Form, Router,
};
use colors_transform::{Color, Rgb};
use serde::Deserialize;

pub async fn setup() {
    let app = Router::new()
        .route("/", get(root))
        .route("/update_lamp", post(update_lamp));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(include_str!("www/index.html"))
}

#[derive(Deserialize, Debug)]
struct UpdateLamp {
    brightness: f32,
    color: String,
}
async fn update_lamp(Form(request): Form<UpdateLamp>) -> Redirect {
    let color = convert_color(Rgb::from_hex_str(&request.color).unwrap());
    let brightness = ((request.brightness / 100.0) * 255.0) as u8;
    let state = if brightness == 0 {
        OnState::Off
    } else {
        OnState::On
    };

    TradfriBulb {
        data: "zigbee2mqtt/0xa46dd4fffe6766fb",
    }
    .publish(TradfriBulbState {
        brightness: Some(brightness),
        color: Some(color),
        state: Some(state),
        ..Default::default()
    })
    .await
    .unwrap();
    Redirect::to("/")
}

// Yoinked from https://stackoverflow.com/a/59875388
fn convert_color(rgb: Rgb) -> Vec2 {
    let red_c = rgb.get_red() / 255.0;
    let green_c = rgb.get_green() / 255.0;
    let blue_c = rgb.get_blue() / 255.0;

    let red_n = if red_c > 0.04045 {
        ((red_c + 0.055) / (1.0 + 0.055)).powf(2.4)
    } else {
        red_c / 12.92
    };
    let green_n = if green_c > 0.04045 {
        ((green_c + 0.055) / (1.0 + 0.055)).powf(2.4)
    } else {
        green_c / 12.92
    };
    let blue_n = if blue_c > 0.04045 {
        ((blue_c + 0.055) / (1.0 + 0.055)).powf(2.4)
    } else {
        blue_c / 12.92
    };

    let x = red_n * 0.664511 + green_n * 0.154324 + blue_n * 0.162028;
    let y = red_n * 0.283881 + green_n * 0.668433 + blue_n * 0.047685;
    let z = red_n * 0.000088 + green_n * 0.072310 + blue_n * 0.986039;

    let x = x / (x + y + z);

    let y = y / (x + y + z);

    Vec2 { x, y }
}

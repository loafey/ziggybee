use futures::future::join_all;

use crate::{
    db::{get_device, get_setup_tree, DeviceType, Endpoint},
    sitegen::draw_object::ToHtml as _,
};

use self::draw_object::{DrawObject, FormField, Input};

mod draw_object;

pub async fn get_html() -> String {
    let content = create_content().await;

    include_str!("index.html").replace("$replace-me-tihi$", &content)
}

async fn create_content() -> String {
    let setup = get_setup_tree().await;
    if setup.unsorted.is_empty() {
        "tihi".to_string()
    } else {
        format!(
            "<div><h1>Sorted</h1><div>{}</div></div><div><h1>Unsorted</h1><div>{}</div></div>",
            setup.setups.draw().await.to_html(),
            setup.unsorted.draw().await.to_html()
        )
    }
}

pub trait Drawable {
    async fn draw(&self) -> DrawObject;
}
impl<T: Drawable> Drawable for Vec<T> {
    async fn draw(&self) -> DrawObject {
        DrawObject::Container {
            title: None,
            children: join_all(self.iter().map(Drawable::draw)).await,
        }
    }
}
impl Drawable for Endpoint {
    async fn draw(&self) -> DrawObject {
        match self {
            Endpoint::Endpoint { name, children } => DrawObject::Container {
                title: Some(name.clone()),
                children: join_all(children.iter().map(Drawable::draw)).await,
            },
            Endpoint::Device { uri } => {
                let device = get_device(uri).await;
                match device {
                    Some(device) => DrawObject::Device {
                        title: device.name.clone(),
                        uri: uri.clone(),
                        fields: match device.r#type {
                            DeviceType::TradfriBulb => vec![
                                FormField {
                                    uri: uri.clone(),
                                    name: "Brightness".to_string(),
                                    form_name: "brightness".to_string(),
                                    input: Input::Slider(0..255),
                                },
                                FormField {
                                    uri: uri.clone(),
                                    name: "Color".to_string(),
                                    form_name: "color".to_string(),
                                    input: Input::Color,
                                },
                                FormField {
                                    uri: uri.clone(),
                                    name: "Temperature".to_string(),
                                    form_name: "color_temp".to_string(),
                                    input: Input::Slider(250..454),
                                },
                                FormField {
                                    uri: uri.clone(),
                                    name: "State".to_string(),
                                    form_name: "state".to_string(),
                                    input: Input::State,
                                },
                            ],
                            DeviceType::TradfriRemoteN2 => vec![],
                            DeviceType::UnknownDevice(_) => vec![],
                        },
                    },
                    None => DrawObject::Device {
                        title: "ERROR DEVICE".to_string(),
                        uri: uri.clone(),
                        fields: vec![],
                    },
                }
            }
        }
    }
}

use crate::{
    db::{data::DeviceType, get_setup, Endpoint},
    sitegen::draw_object::ToHtml as _,
};

use self::draw_object::{DrawObject, FormField, Input};

mod draw_object;

pub async fn get_html() -> String {
    let content = create_content().await;

    include_str!("index.html").replace("$replace-me-tihi$", &content)
}

async fn create_content() -> String {
    let setup = get_setup().await;
    if setup.unsorted.is_empty() {
        "tihi".to_string()
    } else {
        format!(
            "<div><h1>Sorted</h1><div>{}</div></div><div><h1>Unsorted</h1><div>{}</div></div>",
            setup.setups.draw().to_html(),
            setup.unsorted.draw().to_html()
        )
    }
}

pub trait Drawable {
    fn draw(&self) -> DrawObject;
}
impl<T: Drawable> Drawable for Vec<T> {
    fn draw(&self) -> DrawObject {
        DrawObject::Container {
            title: None,
            children: self.iter().map(Drawable::draw).collect(),
        }
    }
}
impl Drawable for Endpoint {
    fn draw(&self) -> DrawObject {
        match self {
            Endpoint::Endpoint { name, children } => DrawObject::Container {
                title: Some(name.clone()),
                children: children.iter().map(Drawable::draw).collect(),
            },
            Endpoint::Device { uri, name, r#type } => DrawObject::Device {
                title: name.clone(),
                uri: uri.clone(),
                fields: match r#type {
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
        }
    }
}

use std::{
    fs::{read_to_string, File},
    io::Write,
    ops::Range,
};

use serde::{Deserialize, Serialize};

fn random_color() -> &'static str {
    use rand::seq::SliceRandom;
    ["--c1", "--c2", "--c4", "--c5"]
        .choose(&mut rand::thread_rng())
        .unwrap()
}

fn main() {
    let a = read_to_string("config.json").unwrap();
    let b = serde_json::from_str::<Vec<Endpoint>>(&a).unwrap();
    let c = b.draw();
    let d = c.to_html();

    let mut f = File::create("dump.html").unwrap();
    f.write_all(
        include_str!("index.html")
            .replace("$replace-me-tihi$", &d)
            .as_bytes(),
    )
    .unwrap();
}

#[derive(Debug)]
pub enum DrawObject {
    Container {
        title: Option<String>,
        children: Vec<DrawObject>,
    },
    Device {
        title: String,
        uri: String,
        fields: Vec<FormField>,
    },
}

pub trait ToHtml {
    fn to_html(&self) -> String;
}
impl ToHtml for DrawObject {
    fn to_html(&self) -> String {
        match self {
            DrawObject::Container { title, children } => {
                let children = children.to_html();
                format!(
                    "<div class=\"container\">{}</div>",
                    if let Some(title) = title {
                        format!(
                            "<h1>{title}</h1><div class=\"container-children\">{children}</div>"
                        )
                    } else {
                        format!("<div class=\"container-children\">{children}</div>")
                    }
                )
            }
            DrawObject::Device { title, uri, fields } => {
                format!(
                    "<div class=\"device\" style=\"--bc: var({});\"><h2>{title}</h2><form>{}</form></div>",
                    random_color(), fields.to_html()
                )
            }
        }
    }
}
impl<T: ToHtml> ToHtml for Vec<T> {
    fn to_html(&self) -> String {
        self.iter().map(|a| a.to_html()).collect()
    }
}
impl ToHtml for FormField {
    fn to_html(&self) -> String {
        format!("<h3>{}</h3><div>{}</div>", self.name, self.input.to_html())
    }
}
impl ToHtml for Input {
    fn to_html(&self) -> String {
        match self {
            Input::Slider(range) => format!(
                "<input type=\"range\" min=\"{}\" max=\"{}\">",
                range.clone().min().unwrap_or_default(),
                range.clone().max().unwrap_or_default()
            ),
            Input::Color => "<input type=\"color\">".to_string(),
            Input::State => format!(
                "{}{}{}{}{}{}",
                "<input type=\"radio\" name=\"state\" id=\"on\">",
                "<label for=\"on\">On</label>",
                "<input type=\"radio\" name=\"state\" id=\"off\">",
                "<label for=\"off\">Off</label>",
                "<input type=\"radio\" name=\"state\" id=\"toggle\">",
                "<label for=\"toggle\">Toggle</label>"
            ),
        }
    }
}

#[derive(Debug)]
pub struct FormField {
    pub name: String,
    pub input: Input,
}

#[derive(Debug)]
pub enum Input {
    Slider(Range<i32>),
    Color,
    State,
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
                            name: "Brightness".to_string(),
                            input: Input::Slider(0..255),
                        },
                        FormField {
                            name: "Color".to_string(),
                            input: Input::Color,
                        },
                        FormField {
                            name: "Temperature".to_string(),
                            input: Input::Slider(250..454),
                        },
                    ],
                },
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Endpoint {
    Device {
        uri: String,
        name: String,
        #[serde(rename = "type")]
        r#type: DeviceType,
    },
    Endpoint {
        name: String,
        #[serde(default)]
        children: Vec<Endpoint>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    TradfriBulb,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum OnState {
    #[default]
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "TOGGLE")]
    Toggle,
}

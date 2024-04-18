use std::ops::Range;

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
                    "<div class=\"device\"><h2>{title}</h2><form class=\"device-form\"><input type=\"hidden\" name=\"uri\" value=\"{uri}\">{}</form></div>",
                    fields.to_html()
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
        let uri = &self.uri;
        let input = match &self.input {
            Input::Slider(range) => format!(
                "<input name=\"{}\" type=\"range\" min=\"{}\" max=\"{}\">",
                self.form_name,
                range.clone().min().unwrap_or_default(),
                range.clone().max().unwrap_or_default()
            ),
            Input::Color => format!("<input name=\"{}\" type=\"color\">", self.form_name),
            #[allow(clippy::format_in_format_args)]
            Input::State => format!(
                "{}{}{}{}{}{}",
                format!(
                    "<input type=\"radio\" name=\"{}\" id=\"{uri}-on\" value=\"on\">",
                    self.form_name
                ),
                format!("<label for=\"{uri}-on\">On</label>"),
                format!(
                    "<input type=\"radio\" name=\"{}\" id=\"{uri}-off\" value=\"off\">",
                    self.form_name
                ),
                format!("<label for=\"off\">Off</label>"),
                format!(
                    "<input type=\"radio\" name=\"{}\" id=\"{uri}-toggle\" value=\"toggle\">",
                    self.form_name
                ),
                format!("<label for=\"{uri}-toggle\">Toggle</label>")
            ),
        };
        format!("<h3>{}</h3>{input}", self.name)
    }
}

#[derive(Debug)]
pub struct FormField {
    pub name: String,
    pub uri: String,
    pub form_name: String,
    pub input: Input,
}

#[derive(Debug)]
pub enum Input {
    Slider(Range<i32>),
    Color,
    State,
}

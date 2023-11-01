use lazy_static::lazy_static;
use log::error;
use serde::Serialize;
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

pub struct Template<'a> {
    path: &'a str,
    context: Context
}

impl<'a> Template<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path,
            context: Context::new(),
        }
    }

    pub fn add<T: Serialize + ?Sized, S: Into<String>>(mut self, key: S, val: &T) -> Self {
        self.context.insert(key, val);
        self
    }

    pub fn render(&mut self) -> String {
        match TEMPLATES.render(self.path, &self.context) {
            Ok(template) => template,
            Err(e) => {
                error!("Error loading template {}: {:?}", self.path, e);
                String::new()
            }
        }
    }
}
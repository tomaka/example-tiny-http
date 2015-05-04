use std::collections::HashMap;
use std::io::Cursor;
use std::string::ToString;
use std::sync::{Arc, Mutex};

use mustache;
use tiny_http;

pub struct TemplatesCache {
    layout: mustache::Template,
    templates: HashMap<String, mustache::Template>,
}

#[derive(Debug, Clone)]
pub enum TemplateError {
    TemplateNotFound { name: String },
}

impl TemplatesCache {
    /// Initializes the list of templates.
    pub fn new() -> TemplatesCache {
        let mut templates = HashMap::new();

        templates.insert("404".to_string(),
                         mustache::compile_str(include_str!("templates/404")));
        templates.insert("user-register".to_string(),
                         mustache::compile_str(include_str!("templates/user-register")));

        TemplatesCache {
            layout: mustache::compile_str(include_str!("templates/main")),
            templates: templates,
        }
    }

    /// Starts composing a template.
    pub fn start(&self, template_name: &str) -> Result<TemplateApplier, TemplateError> {
        let template = match self.templates.get(template_name) {
            Some(t) => t,
            None => return Err(TemplateError::TemplateNotFound { name: template_name.to_string() })
        };

        Ok(TemplateApplier {
            layout: &self.layout,
            template: template,
            map: mustache::MapBuilder::new(),
        })
    }
}

pub struct TemplateApplier<'a> {
    layout: &'a mustache::Template,
    template: &'a mustache::Template,
    map: mustache::MapBuilder,
}

impl<'a> TemplateApplier<'a> {
    /// Finish building the template and return a response.
    pub fn build(self) -> tiny_http::Response<Cursor<Vec<u8>>> {
        let title = Arc::new(Mutex::new(String::new()));
        let head = Arc::new(Mutex::new(String::new()));
        let body = Arc::new(Mutex::new(String::new()));

        let template_data = {
            let (title, head, body) = (title.clone(), head.clone(), body.clone());

            let map = self.map;
            let map = map.insert_fn("block_title", move |input| {
                *title.lock().unwrap() = input;
                format!("")
            });
            let map = map.insert_fn("block_head", move |input| {
                *head.lock().unwrap() = input;
                format!("")
            });
            let map = map.insert_fn("block_body", move |input| {
                *body.lock().unwrap() = input;
                format!("")
            });

            map.build()
        };

        let mut throwaway = Vec::new();
        self.template.render_data(&mut throwaway, &template_data);

        let layout_data = mustache::MapBuilder::new()
            .insert_str("block_title", title.lock().unwrap().clone())
            .insert_str("block_head", head.lock().unwrap().clone())
            .insert_str("block_body", body.lock().unwrap().clone())
            .build();

        let mut output = Vec::new();
        self.layout.render_data(&mut output, &layout_data);

        tiny_http::Response::from_data(output)
    }

    /// Adds a string.
    pub fn insert_str<K: ToString, V: ToString>(mut self, key: K, value: V) -> TemplateApplier<'a> {
        self.map = self.map.insert_str(key, value);
        self
    }
}

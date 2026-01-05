use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Link {
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Link {
    pub fn new(href: &str) -> Self {
        Self {
            href: href.to_string(),
            method: None,
            title: None,
        }
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

pub type Links = HashMap<String, Link>;

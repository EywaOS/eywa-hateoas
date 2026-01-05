use serde::Serialize;
use std::collections::HashMap;
use utoipa::ToSchema;

use super::link::Link;

pub type Links = HashMap<String, Link>;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HateoasResponse<T> {
    pub data: T,
    pub links: Links,
}

impl<T> HateoasResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            links: HashMap::new(),
        }
    }

    pub fn add_link(mut self, rel: &str, link: Link) -> Self {
        self.links.insert(rel.to_string(), link);
        self
    }

    pub fn add_self(mut self, href: &str) -> Self {
        self.links.insert("self".to_string(), Link::new(href));
        self
    }

    pub fn add_collection(mut self, href: &str) -> Self {
        self.links.insert("collection".to_string(), Link::new(href));
        self
    }

    pub fn add_create(mut self, href: &str) -> Self {
        self.links.insert(
            "create".to_string(),
            Link::new(href).method("POST"),
        );
        self
    }

    pub fn add_update(mut self, href: &str) -> Self {
        self.links.insert(
            "update".to_string(),
            Link::new(href).method("PATCH"),
        );
        self
    }

    pub fn add_delete(mut self, href: &str) -> Self {
        self.links.insert(
            "delete".to_string(),
            Link::new(href).method("DELETE"),
        );
        self
    }
}

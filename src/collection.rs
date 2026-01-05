use eywa_pagination::PaginationParams;
use serde::Serialize;
use std::collections::HashMap;
use utoipa::ToSchema;

use super::link::Link;

pub type Links = HashMap<String, Link>;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CollectionMeta {
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CollectionResponse<T> {
    pub data: Vec<T>,
    pub links: Links,
    pub meta: CollectionMeta,
}

impl<T> CollectionResponse<T> {
    pub fn new(data: Vec<T>, pagination: &PaginationParams) -> Self {
        Self {
            data,
            links: HashMap::new(),
            meta: CollectionMeta {
                total: pagination.total,
                page: pagination.page,
                limit: pagination.limit,
                total_pages: pagination.total_pages,
            },
        }
    }

    pub fn add_link(mut self, rel: &str, link: Link) -> Self {
        self.links.insert(rel.to_string(), link);
        self
    }

    pub fn with_pagination_links(mut self, base_url: &str, pagination: &PaginationParams) -> Self {
        self.links.insert(
            "self".to_string(),
            Link::new(&pagination.current_page_url(base_url)),
        );
        self.links.insert(
            "first".to_string(),
            Link::new(&pagination.first_page_url(base_url)),
        );

        if pagination.has_prev {
            self.links.insert(
                "prev".to_string(),
                Link::new(&pagination.prev_page_url(base_url)),
            );
        }

        if pagination.has_next {
            self.links.insert(
                "next".to_string(),
                Link::new(&pagination.next_page_url(base_url)),
            );
        }

        if pagination.has_last {
            self.links.insert(
                "last".to_string(),
                Link::new(&pagination.last_page_url(base_url)),
            );
        }

        self
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub enum PaginateOrder {
    DateTimeDescent,
    #[allow(dead_code)]
    DateTimeAscent,
}

#[derive(Clone)]
pub struct Pagination {
    pub index: usize,
    pub size: usize,
    pub q: Option<String>,
    pub order: PaginateOrder,
}

impl Pagination {
    pub fn new(index: usize, size: usize, order: PaginateOrder) -> Self {
        Self { index, size, order, q: None }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostThumbnail {
    pub title: String,
    pub id: String,
    pub post_date_time: String,
    pub solved: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub post_date_time: String,
    pub solved: bool,
    pub author: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub post_date_time: String,
    pub accepted: bool,
    pub author: String,
    pub content: String,
}

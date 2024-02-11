
use crate::data::{Comment, Pagination, Post, PostThumbnail};
use crate::mocks::PostThumbnailMock;
use gloo::timers::future::TimeoutFuture;

pub async fn get_post_list(pagination: Pagination) -> Vec<PostThumbnail> {
    _ = pagination;
    
    TimeoutFuture::new(1_000).await;
    PostThumbnailMock::get_list()
}

pub async fn get_hot_post_list() -> Vec<PostThumbnail> {
    TimeoutFuture::new(1_000).await;
    PostThumbnailMock::get_hot_post_list()
}

pub async fn get_post(id: String) -> Post {
    TimeoutFuture::new(1_000).await;
    PostThumbnailMock::get_post(&id)
}

pub async fn get_comments(post_id: String) -> Vec<Comment> {
    PostThumbnailMock::get_comments(post_id)
}
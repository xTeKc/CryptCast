use super::{request_delete, request_get, request_post};
use crate::error::Error;
use crate::types::*;

pub async fn create(
    slug: String,
    comment: CommentCreateInfoWrapper,
) -> Result<CommentInfoWrapper, Error> {
    request_post::<CommentCreateInfoWrapper, CommentInfoWrapper>(
        format!("/podcasts/{}/comments", slug),
        comment,
    )
    .await
}

pub async fn delete(slug: String, comment_id: u32) -> Result<DeleteWrapper, Error> {
    request_delete::<DeleteWrapper>(format!("/podcasts/{}/comments/{}", slug, comment_id)).await
}

pub async fn for_podcast(slug: String) -> Result<CommentListInfo, Error> {
    request_get::<CommentListInfo>(format!("/podcasts/{}/comments", slug)).await
}

use super::{limit, request_delete, request_get, request_post, request_put};
use crate::error::Error;
use crate::types::*;

/// Get all podcasts
pub async fn all(page: u32) -> Result<PodcastListInfo, Error> {
    request_get::<PodcastListInfo>(format!("/podcasts?{}", limit(10, page))).await
}

/// Get podcasts filtered by author
pub async fn by_author(author: String, page: u32) -> Result<PodcastListInfo, Error> {
    request_get::<PodcastListInfo>(format!("/podcasts?author={}&{}", author, limit(10, page))).await
}

/// Get podcasts filtered by tag
pub async fn by_tag(tag: String, page: u32) -> Result<PodcastListInfo, Error> {
    request_get::<PodcastListInfo>(format!("/podcasts?tag={}&{}", tag, limit(10, page))).await
}

/// Delete a podcast
pub async fn del(slug: String) -> Result<DeleteWrapper, Error> {
    request_delete::<DeleteWrapper>(format!("/podcasts/{}", slug)).await
}

/// Favorite and podcast
pub async fn favorite(slug: String) -> Result<PodcastInfoWrapper, Error> {
    request_post::<(), PodcastInfoWrapper>(format!("/podcasts/{}/favorite", slug), ()).await
}

/// Unfavorite a podcast
pub async fn unfavorite(slug: String) -> Result<PodcastInfoWrapper, Error> {
    request_delete::<PodcastInfoWrapper>(format!("/podcasts/{}/favorite", slug)).await
}

/// Get podcasts favorited by an author
pub async fn favorited_by(author: String, page: u32) -> Result<PodcastListInfo, Error> {
    request_get::<PodcastListInfo>(format!(
        "/podcasts?favorited={}&{}",
        author,
        limit(10, page)
    ))
    .await
}

/// Get feed of podcasts
pub async fn feed() -> Result<PodcastListInfo, Error> {
    request_get::<PodcastListInfo>(format!("/podcasts/feed?{}", limit(10, 0))).await
}

/// Get a podcast
pub async fn get(slug: String) -> Result<PodcastInfoWrapper, Error> {
    request_get::<PodcastInfoWrapper>(format!("/podcasts/{}", slug)).await
}

/// Update a podcast
pub async fn update(
    slug: String,
    podcast: PodcastCreateUpdateInfoWrapper,
) -> Result<PodcastInfoWrapper, Error> {
    request_put::<PodcastCreateUpdateInfoWrapper, PodcastInfoWrapper>(
        format!("/podcasts/{}", slug),
        Podcast,
    )
    .await
}

/// Create a podcast
pub async fn create(podcast: PodcastCreateUpdateInfoWrapper) -> Result<PodcastInfoWrapper, Error> {
    request_post::<PodcastCreateUpdateInfoWrapper, PodcastInfoWrapper>(
        "/podcasts".to_string(),
        podcast,
    )
    .await
}

//! Common types

mod podcasts;
mod auth;
mod comments;
mod profiles;
mod tags;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use podcasts::{
    PodcastCreateUpdateInfo, PodcastCreateUpdateInfoWrapper, PodcastInfo, PodcastInfoWrapper,
    PodcastListInfo,
};
pub use auth::{
    LoginInfo, LoginInfoWrapper, RegisterInfo, RegisterInfoWrapper, UserInfo, UserInfoWrapper,
    UserUpdateInfo, UserUpdateInfoWrapper,
};
pub use comments::{
    CommentCreateInfo, CommentCreateInfoWrapper, CommentInfo, CommentInfoWrapper, CommentListInfo,
};
pub use profiles::{ProfileInfo, ProfileInfoWrapper};
pub use tags::TagListInfo;

/// cryptcast api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;

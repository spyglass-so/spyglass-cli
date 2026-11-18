pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCommentsResponsePostsItemCommentsItem {
    #[serde(rename = "platformId")]
    #[serde(default)]
    pub platform_id: String,
    #[serde(default)]
    pub text: String,
    #[serde(rename = "likeCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub like_count: f64,
    #[serde(rename = "replyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub reply_count: Option<f64>,
    #[serde(rename = "publishedAt")]
    #[serde(default)]
    pub published_at: String,
    #[serde(rename = "isPinned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    #[serde(default)]
    pub username: String,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "isVerified")]
    #[serde(default)]
    pub is_verified: bool,
    #[serde(rename = "avatarUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl GetCommentsResponsePostsItemCommentsItem {
    pub fn builder() -> GetCommentsResponsePostsItemCommentsItemBuilder {
        <GetCommentsResponsePostsItemCommentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCommentsResponsePostsItemCommentsItemBuilder {
    platform_id: Option<String>,
    text: Option<String>,
    like_count: Option<f64>,
    reply_count: Option<f64>,
    published_at: Option<String>,
    is_pinned: Option<bool>,
    username: Option<String>,
    display_name: Option<String>,
    is_verified: Option<bool>,
    avatar_url: Option<String>,
}

impl GetCommentsResponsePostsItemCommentsItemBuilder {
    pub fn platform_id(mut self, value: impl Into<String>) -> Self {
        self.platform_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn like_count(mut self, value: f64) -> Self {
        self.like_count = Some(value);
        self
    }

    pub fn reply_count(mut self, value: f64) -> Self {
        self.reply_count = Some(value);
        self
    }

    pub fn published_at(mut self, value: impl Into<String>) -> Self {
        self.published_at = Some(value.into());
        self
    }

    pub fn is_pinned(mut self, value: bool) -> Self {
        self.is_pinned = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn is_verified(mut self, value: bool) -> Self {
        self.is_verified = Some(value);
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCommentsResponsePostsItemCommentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform_id`](GetCommentsResponsePostsItemCommentsItemBuilder::platform_id)
    /// - [`text`](GetCommentsResponsePostsItemCommentsItemBuilder::text)
    /// - [`like_count`](GetCommentsResponsePostsItemCommentsItemBuilder::like_count)
    /// - [`published_at`](GetCommentsResponsePostsItemCommentsItemBuilder::published_at)
    /// - [`username`](GetCommentsResponsePostsItemCommentsItemBuilder::username)
    /// - [`display_name`](GetCommentsResponsePostsItemCommentsItemBuilder::display_name)
    /// - [`is_verified`](GetCommentsResponsePostsItemCommentsItemBuilder::is_verified)
    pub fn build(self) -> Result<GetCommentsResponsePostsItemCommentsItem, BuildError> {
        Ok(GetCommentsResponsePostsItemCommentsItem {
            platform_id: self.platform_id.ok_or_else(|| BuildError::missing_field("platform_id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            like_count: self.like_count.ok_or_else(|| BuildError::missing_field("like_count"))?,
            reply_count: self.reply_count,
            published_at: self.published_at.ok_or_else(|| BuildError::missing_field("published_at"))?,
            is_pinned: self.is_pinned,
            username: self.username.ok_or_else(|| BuildError::missing_field("username"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            is_verified: self.is_verified.ok_or_else(|| BuildError::missing_field("is_verified"))?,
            avatar_url: self.avatar_url,
        })
    }
}

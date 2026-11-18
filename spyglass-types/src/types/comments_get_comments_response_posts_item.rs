pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCommentsResponsePostsItem {
    #[serde(default)]
    pub platform: String,
    #[serde(rename = "platformPostId")]
    #[serde(default)]
    pub platform_post_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub comments: Vec<GetCommentsResponsePostsItemCommentsItem>,
}

impl GetCommentsResponsePostsItem {
    pub fn builder() -> GetCommentsResponsePostsItemBuilder {
        <GetCommentsResponsePostsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCommentsResponsePostsItemBuilder {
    platform: Option<String>,
    platform_post_id: Option<String>,
    error: Option<String>,
    comments: Option<Vec<GetCommentsResponsePostsItemCommentsItem>>,
}

impl GetCommentsResponsePostsItemBuilder {
    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn platform_post_id(mut self, value: impl Into<String>) -> Self {
        self.platform_post_id = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn comments(mut self, value: Vec<GetCommentsResponsePostsItemCommentsItem>) -> Self {
        self.comments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCommentsResponsePostsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](GetCommentsResponsePostsItemBuilder::platform)
    /// - [`platform_post_id`](GetCommentsResponsePostsItemBuilder::platform_post_id)
    /// - [`comments`](GetCommentsResponsePostsItemBuilder::comments)
    pub fn build(self) -> Result<GetCommentsResponsePostsItem, BuildError> {
        Ok(GetCommentsResponsePostsItem {
            platform: self.platform.ok_or_else(|| BuildError::missing_field("platform"))?,
            platform_post_id: self.platform_post_id.ok_or_else(|| BuildError::missing_field("platform_post_id"))?,
            error: self.error,
            comments: self.comments.ok_or_else(|| BuildError::missing_field("comments"))?,
        })
    }
}

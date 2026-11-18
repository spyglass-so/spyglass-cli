pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostsAccountResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub media: Vec<PostsAccountResponseMediaItem>,
}

impl PostsAccountResponse {
    pub fn builder() -> PostsAccountResponseBuilder {
        <PostsAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostsAccountResponseBuilder {
    count: Option<i64>,
    media: Option<Vec<PostsAccountResponseMediaItem>>,
}

impl PostsAccountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn media(mut self, value: Vec<PostsAccountResponseMediaItem>) -> Self {
        self.media = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostsAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](PostsAccountResponseBuilder::count)
    /// - [`media`](PostsAccountResponseBuilder::media)
    pub fn build(self) -> Result<PostsAccountResponse, BuildError> {
        Ok(PostsAccountResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            media: self.media.ok_or_else(|| BuildError::missing_field("media"))?,
        })
    }
}

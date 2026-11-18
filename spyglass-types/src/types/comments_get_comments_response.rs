pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCommentsResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub posts: Vec<GetCommentsResponsePostsItem>,
}

impl GetCommentsResponse {
    pub fn builder() -> GetCommentsResponseBuilder {
        <GetCommentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCommentsResponseBuilder {
    count: Option<i64>,
    posts: Option<Vec<GetCommentsResponsePostsItem>>,
}

impl GetCommentsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn posts(mut self, value: Vec<GetCommentsResponsePostsItem>) -> Self {
        self.posts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCommentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](GetCommentsResponseBuilder::count)
    /// - [`posts`](GetCommentsResponseBuilder::posts)
    pub fn build(self) -> Result<GetCommentsResponse, BuildError> {
        Ok(GetCommentsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            posts: self.posts.ok_or_else(|| BuildError::missing_field("posts"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchMediaResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub media: Vec<SearchMediaResponseMediaItem>,
}

impl SearchMediaResponse {
    pub fn builder() -> SearchMediaResponseBuilder {
        <SearchMediaResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMediaResponseBuilder {
    count: Option<i64>,
    media: Option<Vec<SearchMediaResponseMediaItem>>,
}

impl SearchMediaResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn media(mut self, value: Vec<SearchMediaResponseMediaItem>) -> Self {
        self.media = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchMediaResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](SearchMediaResponseBuilder::count)
    /// - [`media`](SearchMediaResponseBuilder::media)
    pub fn build(self) -> Result<SearchMediaResponse, BuildError> {
        Ok(SearchMediaResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            media: self.media.ok_or_else(|| BuildError::missing_field("media"))?,
        })
    }
}

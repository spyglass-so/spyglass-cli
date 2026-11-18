pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IndexAccountRequest {
    #[serde(rename = "websiteUrl")]
    #[serde(default)]
    pub website_url: String,
}

impl IndexAccountRequest {
    pub fn builder() -> IndexAccountRequestBuilder {
        <IndexAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IndexAccountRequestBuilder {
    website_url: Option<String>,
}

impl IndexAccountRequestBuilder {
    pub fn website_url(mut self, value: impl Into<String>) -> Self {
        self.website_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IndexAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`website_url`](IndexAccountRequestBuilder::website_url)
    pub fn build(self) -> Result<IndexAccountRequest, BuildError> {
        Ok(IndexAccountRequest {
            website_url: self.website_url.ok_or_else(|| BuildError::missing_field("website_url"))?,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchQueryRequest {
    #[serde(default)]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl SearchQueryRequest {
    pub fn builder() -> SearchQueryRequestBuilder {
        <SearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchQueryRequestBuilder {
    query: Option<String>,
    limit: Option<i64>,
}

impl SearchQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchQueryRequestBuilder::query)
    pub fn build(self) -> Result<SearchQueryRequest, BuildError> {
        Ok(SearchQueryRequest {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            limit: self.limit,
        })
    }
}


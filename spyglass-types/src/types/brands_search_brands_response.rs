pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchBrandsResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub brands: Vec<SearchBrandsResponseBrandsItem>,
    #[serde(rename = "unmatchedIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmatched_ids: Option<Vec<String>>,
}

impl SearchBrandsResponse {
    pub fn builder() -> SearchBrandsResponseBuilder {
        <SearchBrandsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchBrandsResponseBuilder {
    count: Option<i64>,
    brands: Option<Vec<SearchBrandsResponseBrandsItem>>,
    unmatched_ids: Option<Vec<String>>,
}

impl SearchBrandsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn brands(mut self, value: Vec<SearchBrandsResponseBrandsItem>) -> Self {
        self.brands = Some(value);
        self
    }

    pub fn unmatched_ids(mut self, value: Vec<String>) -> Self {
        self.unmatched_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchBrandsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](SearchBrandsResponseBuilder::count)
    /// - [`brands`](SearchBrandsResponseBuilder::brands)
    pub fn build(self) -> Result<SearchBrandsResponse, BuildError> {
        Ok(SearchBrandsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            brands: self.brands.ok_or_else(|| BuildError::missing_field("brands"))?,
            unmatched_ids: self.unmatched_ids,
        })
    }
}

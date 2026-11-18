pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchCreatorsResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub creators: Vec<SearchCreatorsResponseCreatorsItem>,
    #[serde(rename = "unmatchedIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmatched_ids: Option<Vec<String>>,
}

impl SearchCreatorsResponse {
    pub fn builder() -> SearchCreatorsResponseBuilder {
        <SearchCreatorsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchCreatorsResponseBuilder {
    count: Option<i64>,
    creators: Option<Vec<SearchCreatorsResponseCreatorsItem>>,
    unmatched_ids: Option<Vec<String>>,
}

impl SearchCreatorsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn creators(mut self, value: Vec<SearchCreatorsResponseCreatorsItem>) -> Self {
        self.creators = Some(value);
        self
    }

    pub fn unmatched_ids(mut self, value: Vec<String>) -> Self {
        self.unmatched_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchCreatorsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](SearchCreatorsResponseBuilder::count)
    /// - [`creators`](SearchCreatorsResponseBuilder::creators)
    pub fn build(self) -> Result<SearchCreatorsResponse, BuildError> {
        Ok(SearchCreatorsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            creators: self.creators.ok_or_else(|| BuildError::missing_field("creators"))?,
            unmatched_ids: self.unmatched_ids,
        })
    }
}

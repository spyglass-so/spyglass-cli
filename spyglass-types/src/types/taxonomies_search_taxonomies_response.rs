pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchTaxonomiesResponse {
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub count: i64,
}

impl SearchTaxonomiesResponse {
    pub fn builder() -> SearchTaxonomiesResponseBuilder {
        <SearchTaxonomiesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchTaxonomiesResponseBuilder {
    paths: Option<Vec<String>>,
    count: Option<i64>,
}

impl SearchTaxonomiesResponseBuilder {
    pub fn paths(mut self, value: Vec<String>) -> Self {
        self.paths = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchTaxonomiesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paths`](SearchTaxonomiesResponseBuilder::paths)
    /// - [`count`](SearchTaxonomiesResponseBuilder::count)
    pub fn build(self) -> Result<SearchTaxonomiesResponse, BuildError> {
        Ok(SearchTaxonomiesResponse {
            paths: self.paths.ok_or_else(|| BuildError::missing_field("paths"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}

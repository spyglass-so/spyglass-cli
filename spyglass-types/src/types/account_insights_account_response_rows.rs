pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InsightsAccountResponseRows {
    pub r#type: InsightsAccountResponseRowsType,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub rows: Vec<InsightsAccountResponseRowsRowsItem>,
}

impl InsightsAccountResponseRows {
    pub fn builder() -> InsightsAccountResponseRowsBuilder {
        <InsightsAccountResponseRowsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsightsAccountResponseRowsBuilder {
    r#type: Option<InsightsAccountResponseRowsType>,
    count: Option<i64>,
    rows: Option<Vec<InsightsAccountResponseRowsRowsItem>>,
}

impl InsightsAccountResponseRowsBuilder {
    pub fn r#type(mut self, value: InsightsAccountResponseRowsType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<InsightsAccountResponseRowsRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsightsAccountResponseRows`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InsightsAccountResponseRowsBuilder::r#type)
    /// - [`count`](InsightsAccountResponseRowsBuilder::count)
    /// - [`rows`](InsightsAccountResponseRowsBuilder::rows)
    pub fn build(self) -> Result<InsightsAccountResponseRows, BuildError> {
        Ok(InsightsAccountResponseRows {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

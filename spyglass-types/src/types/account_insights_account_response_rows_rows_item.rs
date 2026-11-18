pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InsightsAccountResponseRowsRowsItem {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub count: i64,
}

impl InsightsAccountResponseRowsRowsItem {
    pub fn builder() -> InsightsAccountResponseRowsRowsItemBuilder {
        <InsightsAccountResponseRowsRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsightsAccountResponseRowsRowsItemBuilder {
    r#type: Option<String>,
    count: Option<i64>,
}

impl InsightsAccountResponseRowsRowsItemBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsightsAccountResponseRowsRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InsightsAccountResponseRowsRowsItemBuilder::r#type)
    /// - [`count`](InsightsAccountResponseRowsRowsItemBuilder::count)
    pub fn build(self) -> Result<InsightsAccountResponseRowsRowsItem, BuildError> {
        Ok(InsightsAccountResponseRowsRowsItem {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}

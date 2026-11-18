pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InsightsAccountResponseStatus {
    pub r#type: InsightsAccountResponseStatusType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InsightsAccountResponseStatusStatus>,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub totals: Vec<InsightsAccountResponseStatusTotalsItem>,
}

impl InsightsAccountResponseStatus {
    pub fn builder() -> InsightsAccountResponseStatusBuilder {
        <InsightsAccountResponseStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsightsAccountResponseStatusBuilder {
    r#type: Option<InsightsAccountResponseStatusType>,
    status: Option<InsightsAccountResponseStatusStatus>,
    count: Option<i64>,
    totals: Option<Vec<InsightsAccountResponseStatusTotalsItem>>,
}

impl InsightsAccountResponseStatusBuilder {
    pub fn r#type(mut self, value: InsightsAccountResponseStatusType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: InsightsAccountResponseStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<InsightsAccountResponseStatusTotalsItem>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsightsAccountResponseStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InsightsAccountResponseStatusBuilder::r#type)
    /// - [`count`](InsightsAccountResponseStatusBuilder::count)
    /// - [`totals`](InsightsAccountResponseStatusBuilder::totals)
    pub fn build(self) -> Result<InsightsAccountResponseStatus, BuildError> {
        Ok(InsightsAccountResponseStatus {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            status: self.status,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            totals: self.totals.ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}

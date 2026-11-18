pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for insights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InsightsQueryRequest {
    pub r#type: InsightsAccountRequestType,
    #[serde(rename = "maxDaysBack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_days_back: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl InsightsQueryRequest {
    pub fn builder() -> InsightsQueryRequestBuilder {
        <InsightsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsightsQueryRequestBuilder {
    r#type: Option<InsightsAccountRequestType>,
    max_days_back: Option<i64>,
    limit: Option<i64>,
}

impl InsightsQueryRequestBuilder {
    pub fn r#type(mut self, value: InsightsAccountRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn max_days_back(mut self, value: i64) -> Self {
        self.max_days_back = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsightsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InsightsQueryRequestBuilder::r#type)
    pub fn build(self) -> Result<InsightsQueryRequest, BuildError> {
        Ok(InsightsQueryRequest {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            max_days_back: self.max_days_back,
            limit: self.limit,
        })
    }
}


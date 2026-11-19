pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for insights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InsightsQueryRequest {
    pub r#type: InsightsAccountRequestType,
    #[serde(rename = "datePreset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_preset: Option<InsightsAccountRequestDatePreset>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
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
    date_preset: Option<InsightsAccountRequestDatePreset>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    limit: Option<i64>,
}

impl InsightsQueryRequestBuilder {
    pub fn r#type(mut self, value: InsightsAccountRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn date_preset(mut self, value: InsightsAccountRequestDatePreset) -> Self {
        self.date_preset = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
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
            date_preset: self.date_preset,
            start_date: self.start_date,
            end_date: self.end_date,
            limit: self.limit,
        })
    }
}


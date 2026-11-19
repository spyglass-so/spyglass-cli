pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for collabs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "datePreset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_preset: Option<CollabsAccountRequestDatePreset>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
}

impl CollabsQueryRequest {
    pub fn builder() -> CollabsQueryRequestBuilder {
        <CollabsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    date_preset: Option<CollabsAccountRequestDatePreset>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl CollabsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn date_preset(mut self, value: CollabsAccountRequestDatePreset) -> Self {
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

    /// Consumes the builder and constructs a [`CollabsQueryRequest`].
    pub fn build(self) -> Result<CollabsQueryRequest, BuildError> {
        Ok(CollabsQueryRequest {
            limit: self.limit,
            offset: self.offset,
            date_preset: self.date_preset,
            start_date: self.start_date,
            end_date: self.end_date,
        })
    }
}


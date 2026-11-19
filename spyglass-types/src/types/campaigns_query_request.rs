pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for campaigns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CampaignsQueryRequest {
    pub r#type: CampaignsAccountRequestType,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<CampaignsAccountRequestSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "datePreset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_preset: Option<CampaignsAccountRequestDatePreset>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
}

impl CampaignsQueryRequest {
    pub fn builder() -> CampaignsQueryRequestBuilder {
        <CampaignsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsQueryRequestBuilder {
    r#type: Option<CampaignsAccountRequestType>,
    sort_by: Option<CampaignsAccountRequestSortBy>,
    limit: Option<i64>,
    date_preset: Option<CampaignsAccountRequestDatePreset>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl CampaignsQueryRequestBuilder {
    pub fn r#type(mut self, value: CampaignsAccountRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn sort_by(mut self, value: CampaignsAccountRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn date_preset(mut self, value: CampaignsAccountRequestDatePreset) -> Self {
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

    /// Consumes the builder and constructs a [`CampaignsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CampaignsQueryRequestBuilder::r#type)
    pub fn build(self) -> Result<CampaignsQueryRequest, BuildError> {
        Ok(CampaignsQueryRequest {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            sort_by: self.sort_by,
            limit: self.limit,
            date_preset: self.date_preset,
            start_date: self.start_date,
            end_date: self.end_date,
        })
    }
}


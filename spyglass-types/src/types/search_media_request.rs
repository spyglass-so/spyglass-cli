pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchMediaRequest {
    /// Account id given as a string, uuid, or handle
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<SearchMediaRequestAccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Vec<SearchMediaRequestChannelItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Vec<SearchMediaRequestFormatItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<SearchMediaRequestDurationItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SearchMediaRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<SearchMediaRequestCategoriesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<Vec<String>>,
    #[serde(rename = "funnelStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnel_stage: Option<Vec<SearchMediaRequestFunnelStageItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<SearchMediaRequestLanguageItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    #[serde(rename = "datePreset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_preset: Option<SearchMediaRequestDatePreset>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Calendar date, YYYY-MM-DD.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "minDaysRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_days_run: Option<f64>,
    #[serde(rename = "maxDaysRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_days_run: Option<f64>,
    #[serde(rename = "minViewCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_view_count: Option<f64>,
    #[serde(rename = "maxViewCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_view_count: Option<f64>,
    #[serde(rename = "landingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page: Option<String>,
    #[serde(rename = "landingPageMatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page_match: Option<SearchMediaRequestLandingPageMatch>,
    #[serde(rename = "lowImpressions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_impressions: Option<bool>,
    #[serde(rename = "partnershipAd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partnership_ad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossposted: Option<bool>,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SearchMediaRequestSortBy>,
    /// Include each video transcript. Does not change the credit charge. Default false.
    #[serde(rename = "includeTranscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transcript: Option<bool>,
    /// Include each post caption. Does not change the credit charge. Default false.
    #[serde(rename = "includeCaption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_caption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl SearchMediaRequest {
    pub fn builder() -> SearchMediaRequestBuilder {
        <SearchMediaRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMediaRequestBuilder {
    account_id: Option<SearchMediaRequestAccountId>,
    query: Option<String>,
    channel: Option<Vec<SearchMediaRequestChannelItem>>,
    format: Option<Vec<SearchMediaRequestFormatItem>>,
    duration: Option<Vec<SearchMediaRequestDurationItem>>,
    status: Option<SearchMediaRequestStatus>,
    categories: Option<Vec<SearchMediaRequestCategoriesItem>>,
    taxonomy: Option<Vec<String>>,
    funnel_stage: Option<Vec<SearchMediaRequestFunnelStageItem>>,
    language: Option<Vec<SearchMediaRequestLanguageItem>>,
    location: Option<Vec<String>>,
    date_preset: Option<SearchMediaRequestDatePreset>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    min_days_run: Option<f64>,
    max_days_run: Option<f64>,
    min_view_count: Option<f64>,
    max_view_count: Option<f64>,
    landing_page: Option<String>,
    landing_page_match: Option<SearchMediaRequestLandingPageMatch>,
    low_impressions: Option<bool>,
    partnership_ad: Option<bool>,
    crossposted: Option<bool>,
    sort_by: Option<SearchMediaRequestSortBy>,
    include_transcript: Option<bool>,
    include_caption: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl SearchMediaRequestBuilder {
    pub fn account_id(mut self, value: SearchMediaRequestAccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn channel(mut self, value: Vec<SearchMediaRequestChannelItem>) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn format(mut self, value: Vec<SearchMediaRequestFormatItem>) -> Self {
        self.format = Some(value);
        self
    }

    pub fn duration(mut self, value: Vec<SearchMediaRequestDurationItem>) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn status(mut self, value: SearchMediaRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn categories(mut self, value: Vec<SearchMediaRequestCategoriesItem>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn taxonomy(mut self, value: Vec<String>) -> Self {
        self.taxonomy = Some(value);
        self
    }

    pub fn funnel_stage(mut self, value: Vec<SearchMediaRequestFunnelStageItem>) -> Self {
        self.funnel_stage = Some(value);
        self
    }

    pub fn language(mut self, value: Vec<SearchMediaRequestLanguageItem>) -> Self {
        self.language = Some(value);
        self
    }

    pub fn location(mut self, value: Vec<String>) -> Self {
        self.location = Some(value);
        self
    }

    pub fn date_preset(mut self, value: SearchMediaRequestDatePreset) -> Self {
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

    pub fn min_days_run(mut self, value: f64) -> Self {
        self.min_days_run = Some(value);
        self
    }

    pub fn max_days_run(mut self, value: f64) -> Self {
        self.max_days_run = Some(value);
        self
    }

    pub fn min_view_count(mut self, value: f64) -> Self {
        self.min_view_count = Some(value);
        self
    }

    pub fn max_view_count(mut self, value: f64) -> Self {
        self.max_view_count = Some(value);
        self
    }

    pub fn landing_page(mut self, value: impl Into<String>) -> Self {
        self.landing_page = Some(value.into());
        self
    }

    pub fn landing_page_match(mut self, value: SearchMediaRequestLandingPageMatch) -> Self {
        self.landing_page_match = Some(value);
        self
    }

    pub fn low_impressions(mut self, value: bool) -> Self {
        self.low_impressions = Some(value);
        self
    }

    pub fn partnership_ad(mut self, value: bool) -> Self {
        self.partnership_ad = Some(value);
        self
    }

    pub fn crossposted(mut self, value: bool) -> Self {
        self.crossposted = Some(value);
        self
    }

    pub fn sort_by(mut self, value: SearchMediaRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn include_transcript(mut self, value: bool) -> Self {
        self.include_transcript = Some(value);
        self
    }

    pub fn include_caption(mut self, value: bool) -> Self {
        self.include_caption = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchMediaRequest`].
    pub fn build(self) -> Result<SearchMediaRequest, BuildError> {
        Ok(SearchMediaRequest {
            account_id: self.account_id,
            query: self.query,
            channel: self.channel,
            format: self.format,
            duration: self.duration,
            status: self.status,
            categories: self.categories,
            taxonomy: self.taxonomy,
            funnel_stage: self.funnel_stage,
            language: self.language,
            location: self.location,
            date_preset: self.date_preset,
            start_date: self.start_date,
            end_date: self.end_date,
            min_days_run: self.min_days_run,
            max_days_run: self.max_days_run,
            min_view_count: self.min_view_count,
            max_view_count: self.max_view_count,
            landing_page: self.landing_page,
            landing_page_match: self.landing_page_match,
            low_impressions: self.low_impressions,
            partnership_ad: self.partnership_ad,
            crossposted: self.crossposted,
            sort_by: self.sort_by,
            include_transcript: self.include_transcript,
            include_caption: self.include_caption,
            limit: self.limit,
            offset: self.offset,
        })
    }
}


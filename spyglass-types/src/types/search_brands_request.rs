pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchBrandsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<SearchBrandsRequestIdsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "igUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_username: Option<String>,
    #[serde(rename = "tiktokUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "minFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_followers: Option<i64>,
    #[serde(rename = "maxFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_followers: Option<i64>,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SearchBrandsRequestSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<SearchBrandsRequestLanguage>,
}

impl SearchBrandsRequest {
    pub fn builder() -> SearchBrandsRequestBuilder {
        <SearchBrandsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchBrandsRequestBuilder {
    ids: Option<Vec<SearchBrandsRequestIdsItem>>,
    names: Option<Vec<String>>,
    ig_username: Option<String>,
    tiktok_username: Option<String>,
    query: Option<String>,
    location: Option<String>,
    min_followers: Option<i64>,
    max_followers: Option<i64>,
    sort_by: Option<SearchBrandsRequestSortBy>,
    offset: Option<i64>,
    limit: Option<i64>,
    taxonomy: Option<Vec<String>>,
    language: Option<SearchBrandsRequestLanguage>,
}

impl SearchBrandsRequestBuilder {
    pub fn ids(mut self, value: Vec<SearchBrandsRequestIdsItem>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn names(mut self, value: Vec<String>) -> Self {
        self.names = Some(value);
        self
    }

    pub fn ig_username(mut self, value: impl Into<String>) -> Self {
        self.ig_username = Some(value.into());
        self
    }

    pub fn tiktok_username(mut self, value: impl Into<String>) -> Self {
        self.tiktok_username = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn min_followers(mut self, value: i64) -> Self {
        self.min_followers = Some(value);
        self
    }

    pub fn max_followers(mut self, value: i64) -> Self {
        self.max_followers = Some(value);
        self
    }

    pub fn sort_by(mut self, value: SearchBrandsRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn taxonomy(mut self, value: Vec<String>) -> Self {
        self.taxonomy = Some(value);
        self
    }

    pub fn language(mut self, value: SearchBrandsRequestLanguage) -> Self {
        self.language = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchBrandsRequest`].
    pub fn build(self) -> Result<SearchBrandsRequest, BuildError> {
        Ok(SearchBrandsRequest {
            ids: self.ids,
            names: self.names,
            ig_username: self.ig_username,
            tiktok_username: self.tiktok_username,
            query: self.query,
            location: self.location,
            min_followers: self.min_followers,
            max_followers: self.max_followers,
            sort_by: self.sort_by,
            offset: self.offset,
            limit: self.limit,
            taxonomy: self.taxonomy,
            language: self.language,
        })
    }
}


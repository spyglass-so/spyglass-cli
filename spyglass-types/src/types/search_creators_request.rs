pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchCreatorsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<SearchCreatorsRequestIdsItem>>,
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
    pub sort_by: Option<SearchCreatorsRequestSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<SearchCreatorsRequestEthnicity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<SearchCreatorsRequestGender>,
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<SearchCreatorsRequestAgeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<SearchCreatorsRequestLanguage>,
}

impl SearchCreatorsRequest {
    pub fn builder() -> SearchCreatorsRequestBuilder {
        <SearchCreatorsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchCreatorsRequestBuilder {
    ids: Option<Vec<SearchCreatorsRequestIdsItem>>,
    names: Option<Vec<String>>,
    ig_username: Option<String>,
    tiktok_username: Option<String>,
    query: Option<String>,
    location: Option<String>,
    min_followers: Option<i64>,
    max_followers: Option<i64>,
    sort_by: Option<SearchCreatorsRequestSortBy>,
    offset: Option<i64>,
    limit: Option<i64>,
    ethnicity: Option<SearchCreatorsRequestEthnicity>,
    gender: Option<SearchCreatorsRequestGender>,
    age_range: Option<SearchCreatorsRequestAgeRange>,
    language: Option<SearchCreatorsRequestLanguage>,
}

impl SearchCreatorsRequestBuilder {
    pub fn ids(mut self, value: Vec<SearchCreatorsRequestIdsItem>) -> Self {
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

    pub fn sort_by(mut self, value: SearchCreatorsRequestSortBy) -> Self {
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

    pub fn ethnicity(mut self, value: SearchCreatorsRequestEthnicity) -> Self {
        self.ethnicity = Some(value);
        self
    }

    pub fn gender(mut self, value: SearchCreatorsRequestGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn age_range(mut self, value: SearchCreatorsRequestAgeRange) -> Self {
        self.age_range = Some(value);
        self
    }

    pub fn language(mut self, value: SearchCreatorsRequestLanguage) -> Self {
        self.language = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchCreatorsRequest`].
    pub fn build(self) -> Result<SearchCreatorsRequest, BuildError> {
        Ok(SearchCreatorsRequest {
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
            ethnicity: self.ethnicity,
            gender: self.gender,
            age_range: self.age_range,
            language: self.language,
        })
    }
}


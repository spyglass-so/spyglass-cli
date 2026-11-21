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
    #[serde(rename = "minViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_views: Option<i64>,
    #[serde(rename = "maxViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_views: Option<i64>,
    #[serde(rename = "minEngagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_engagement: Option<f64>,
    #[serde(rename = "maxEngagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_engagement: Option<f64>,
    #[serde(rename = "minActiveAds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_active_ads: Option<i64>,
    #[serde(rename = "maxActiveAds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_ads: Option<i64>,
    /// Creator niche, such as Celebrity or Comedian. Matches any listed value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<SearchCreatorsRequestCategoryItem>>,
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
    min_views: Option<i64>,
    max_views: Option<i64>,
    min_engagement: Option<f64>,
    max_engagement: Option<f64>,
    min_active_ads: Option<i64>,
    max_active_ads: Option<i64>,
    category: Option<Vec<SearchCreatorsRequestCategoryItem>>,
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

    pub fn min_views(mut self, value: i64) -> Self {
        self.min_views = Some(value);
        self
    }

    pub fn max_views(mut self, value: i64) -> Self {
        self.max_views = Some(value);
        self
    }

    pub fn min_engagement(mut self, value: f64) -> Self {
        self.min_engagement = Some(value);
        self
    }

    pub fn max_engagement(mut self, value: f64) -> Self {
        self.max_engagement = Some(value);
        self
    }

    pub fn min_active_ads(mut self, value: i64) -> Self {
        self.min_active_ads = Some(value);
        self
    }

    pub fn max_active_ads(mut self, value: i64) -> Self {
        self.max_active_ads = Some(value);
        self
    }

    pub fn category(mut self, value: Vec<SearchCreatorsRequestCategoryItem>) -> Self {
        self.category = Some(value);
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
            min_views: self.min_views,
            max_views: self.max_views,
            min_engagement: self.min_engagement,
            max_engagement: self.max_engagement,
            min_active_ads: self.min_active_ads,
            max_active_ads: self.max_active_ads,
            category: self.category,
            ethnicity: self.ethnicity,
            gender: self.gender,
            age_range: self.age_range,
            language: self.language,
        })
    }
}


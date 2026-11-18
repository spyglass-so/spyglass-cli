pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchCreatorsResponseCreatorsItem {
    pub r#type: SearchCreatorsResponseCreatorsItemType,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "facebookAdLibraryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_ad_library_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default)]
    pub verified: bool,
    #[serde(rename = "igUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_username: Option<String>,
    #[serde(rename = "igFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ig_followers: Option<f64>,
    #[serde(rename = "igEngagementRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ig_engagement_rate: Option<f64>,
    #[serde(rename = "tiktokUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_username: Option<String>,
    #[serde(rename = "tiktokFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tiktok_followers: Option<f64>,
    #[serde(rename = "tiktokEngagementRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tiktok_engagement_rate: Option<f64>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<String>,
}

impl SearchCreatorsResponseCreatorsItem {
    pub fn builder() -> SearchCreatorsResponseCreatorsItemBuilder {
        <SearchCreatorsResponseCreatorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchCreatorsResponseCreatorsItemBuilder {
    r#type: Option<SearchCreatorsResponseCreatorsItemType>,
    id: Option<String>,
    facebook_ad_library_id: Option<String>,
    name: Option<String>,
    summary: Option<String>,
    bio: Option<String>,
    verified: Option<bool>,
    ig_username: Option<String>,
    ig_followers: Option<f64>,
    ig_engagement_rate: Option<f64>,
    tiktok_username: Option<String>,
    tiktok_followers: Option<f64>,
    tiktok_engagement_rate: Option<f64>,
    thumbnail_url: Option<String>,
    location: Option<String>,
    language: Option<String>,
    ethnicity: Option<String>,
    gender: Option<String>,
    age_range: Option<String>,
}

impl SearchCreatorsResponseCreatorsItemBuilder {
    pub fn r#type(mut self, value: SearchCreatorsResponseCreatorsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn facebook_ad_library_id(mut self, value: impl Into<String>) -> Self {
        self.facebook_ad_library_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn bio(mut self, value: impl Into<String>) -> Self {
        self.bio = Some(value.into());
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn ig_username(mut self, value: impl Into<String>) -> Self {
        self.ig_username = Some(value.into());
        self
    }

    pub fn ig_followers(mut self, value: f64) -> Self {
        self.ig_followers = Some(value);
        self
    }

    pub fn ig_engagement_rate(mut self, value: f64) -> Self {
        self.ig_engagement_rate = Some(value);
        self
    }

    pub fn tiktok_username(mut self, value: impl Into<String>) -> Self {
        self.tiktok_username = Some(value.into());
        self
    }

    pub fn tiktok_followers(mut self, value: f64) -> Self {
        self.tiktok_followers = Some(value);
        self
    }

    pub fn tiktok_engagement_rate(mut self, value: f64) -> Self {
        self.tiktok_engagement_rate = Some(value);
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn ethnicity(mut self, value: impl Into<String>) -> Self {
        self.ethnicity = Some(value.into());
        self
    }

    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(value.into());
        self
    }

    pub fn age_range(mut self, value: impl Into<String>) -> Self {
        self.age_range = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchCreatorsResponseCreatorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SearchCreatorsResponseCreatorsItemBuilder::r#type)
    /// - [`id`](SearchCreatorsResponseCreatorsItemBuilder::id)
    /// - [`name`](SearchCreatorsResponseCreatorsItemBuilder::name)
    /// - [`verified`](SearchCreatorsResponseCreatorsItemBuilder::verified)
    pub fn build(self) -> Result<SearchCreatorsResponseCreatorsItem, BuildError> {
        Ok(SearchCreatorsResponseCreatorsItem {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            facebook_ad_library_id: self.facebook_ad_library_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            summary: self.summary,
            bio: self.bio,
            verified: self.verified.ok_or_else(|| BuildError::missing_field("verified"))?,
            ig_username: self.ig_username,
            ig_followers: self.ig_followers,
            ig_engagement_rate: self.ig_engagement_rate,
            tiktok_username: self.tiktok_username,
            tiktok_followers: self.tiktok_followers,
            tiktok_engagement_rate: self.tiktok_engagement_rate,
            thumbnail_url: self.thumbnail_url,
            location: self.location,
            language: self.language,
            ethnicity: self.ethnicity,
            gender: self.gender,
            age_range: self.age_range,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GetAccountResponse {
        #[serde(rename = "BRAND")]
        #[non_exhaustive]
        Brand {
            #[serde(default)]
            id: String,
            #[serde(rename = "facebookAdLibraryId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            facebook_ad_library_id: Option<String>,
            #[serde(default)]
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            summary: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            bio: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language: Option<String>,
            #[serde(default)]
            verified: bool,
            #[serde(rename = "facebookUsername")]
            #[serde(skip_serializing_if = "Option::is_none")]
            facebook_username: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            likes: Option<f64>,
            #[serde(rename = "igUsername")]
            #[serde(skip_serializing_if = "Option::is_none")]
            ig_username: Option<String>,
            #[serde(rename = "igFollowers")]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            ig_followers: Option<f64>,
            #[serde(rename = "igEngagementRate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            ig_engagement_rate: Option<f64>,
            #[serde(rename = "tiktokUsername")]
            #[serde(skip_serializing_if = "Option::is_none")]
            tiktok_username: Option<String>,
            #[serde(rename = "tiktokFollowers")]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            tiktok_followers: Option<f64>,
            #[serde(rename = "thumbnailUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            thumbnail_url: Option<String>,
        },

        #[serde(rename = "CREATOR")]
        #[non_exhaustive]
        Creator {
            #[serde(default)]
            id: String,
            #[serde(rename = "facebookAdLibraryId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            facebook_ad_library_id: Option<String>,
            #[serde(default)]
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            summary: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            bio: Option<String>,
            #[serde(default)]
            verified: bool,
            #[serde(rename = "igUsername")]
            #[serde(skip_serializing_if = "Option::is_none")]
            ig_username: Option<String>,
            #[serde(rename = "igFollowers")]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            ig_followers: Option<f64>,
            #[serde(rename = "igEngagementRate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            ig_engagement_rate: Option<f64>,
            #[serde(rename = "tiktokUsername")]
            #[serde(skip_serializing_if = "Option::is_none")]
            tiktok_username: Option<String>,
            #[serde(rename = "tiktokFollowers")]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            tiktok_followers: Option<f64>,
            #[serde(rename = "tiktokEngagementRate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            tiktok_engagement_rate: Option<f64>,
            #[serde(rename = "thumbnailUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            thumbnail_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ethnicity: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gender: Option<String>,
            #[serde(rename = "ageRange")]
            #[serde(skip_serializing_if = "Option::is_none")]
            age_range: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl GetAccountResponse {
    pub fn brand(id: String, name: String, verified: bool) -> Self {
        Self::Brand { id, facebook_ad_library_id: None, name, summary: None, bio: None, category: None, location: None, language: None, verified, facebook_username: None, likes: None, ig_username: None, ig_followers: None, ig_engagement_rate: None, tiktok_username: None, tiktok_followers: None, thumbnail_url: None }
    }

    pub fn creator(id: String, name: String, verified: bool) -> Self {
        Self::Creator { id, facebook_ad_library_id: None, name, summary: None, bio: None, verified, ig_username: None, ig_followers: None, ig_engagement_rate: None, tiktok_username: None, tiktok_followers: None, tiktok_engagement_rate: None, thumbnail_url: None, location: None, language: None, ethnicity: None, gender: None, age_range: None }
    }

    pub fn brand_with_facebook_ad_library_id(id: String, facebook_ad_library_id: String, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id: Some(facebook_ad_library_id), name, summary, bio, category, location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_summary(id: String, facebook_ad_library_id: Option<String>, name: String, summary: String, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary: Some(summary), bio, category, location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_bio(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: String, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio: Some(bio), category, location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_category(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: String, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category: Some(category), location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_location(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: String, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location: Some(location), language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_language(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: String, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language: Some(language), verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_facebook_username(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: String, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username: Some(facebook_username), likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_likes(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: f64, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username, likes: Some(likes), ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_ig_username(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: String, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username, likes, ig_username: Some(ig_username), ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_ig_followers(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: f64, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username, likes, ig_username, ig_followers: Some(ig_followers), ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_ig_engagement_rate(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: f64, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate: Some(ig_engagement_rate), tiktok_username, tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_tiktok_username(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: String, tiktok_followers: Option<f64>, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username: Some(tiktok_username), tiktok_followers, thumbnail_url }
    }

    pub fn brand_with_tiktok_followers(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: f64, thumbnail_url: Option<String>) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers: Some(tiktok_followers), thumbnail_url }
    }

    pub fn brand_with_thumbnail_url(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, category: Option<String>, location: Option<String>, language: Option<String>, verified: bool, facebook_username: Option<String>, likes: Option<f64>, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, thumbnail_url: String) -> Self {
        Self::Brand { id, facebook_ad_library_id, name, summary, bio, category, location, language, verified, facebook_username, likes, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, thumbnail_url: Some(thumbnail_url) }
    }

    pub fn creator_with_facebook_ad_library_id(id: String, facebook_ad_library_id: String, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id: Some(facebook_ad_library_id), name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_summary(id: String, facebook_ad_library_id: Option<String>, name: String, summary: String, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary: Some(summary), bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_bio(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: String, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio: Some(bio), verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_ig_username(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: String, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username: Some(ig_username), ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_ig_followers(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: f64, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers: Some(ig_followers), ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_ig_engagement_rate(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: f64, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate: Some(ig_engagement_rate), tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_tiktok_username(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: String, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username: Some(tiktok_username), tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_tiktok_followers(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: f64, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers: Some(tiktok_followers), tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_tiktok_engagement_rate(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: f64, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate: Some(tiktok_engagement_rate), thumbnail_url, location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_thumbnail_url(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: String, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url: Some(thumbnail_url), location, language, ethnicity, gender, age_range }
    }

    pub fn creator_with_location(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: String, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location: Some(location), language, ethnicity, gender, age_range }
    }

    pub fn creator_with_language(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: String, ethnicity: Option<String>, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language: Some(language), ethnicity, gender, age_range }
    }

    pub fn creator_with_ethnicity(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: String, gender: Option<String>, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity: Some(ethnicity), gender, age_range }
    }

    pub fn creator_with_gender(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: String, age_range: Option<String>) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender: Some(gender), age_range }
    }

    pub fn creator_with_age_range(id: String, facebook_ad_library_id: Option<String>, name: String, summary: Option<String>, bio: Option<String>, verified: bool, ig_username: Option<String>, ig_followers: Option<f64>, ig_engagement_rate: Option<f64>, tiktok_username: Option<String>, tiktok_followers: Option<f64>, tiktok_engagement_rate: Option<f64>, thumbnail_url: Option<String>, location: Option<String>, language: Option<String>, ethnicity: Option<String>, gender: Option<String>, age_range: String) -> Self {
        Self::Creator { id, facebook_ad_library_id, name, summary, bio, verified, ig_username, ig_followers, ig_engagement_rate, tiktok_username, tiktok_followers, tiktok_engagement_rate, thumbnail_url, location, language, ethnicity, gender, age_range: Some(age_range) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

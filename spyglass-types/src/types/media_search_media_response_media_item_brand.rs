pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchMediaResponseMediaItemBrand {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "facebookAdLibraryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_ad_library_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SearchMediaResponseMediaItemBrandType>,
    #[serde(rename = "igUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_username: Option<String>,
    #[serde(rename = "igFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_followers: Option<f64>,
    #[serde(rename = "tiktokUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_username: Option<String>,
    #[serde(rename = "tiktokFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_followers: Option<f64>,
    #[serde(rename = "facebookUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_username: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl SearchMediaResponseMediaItemBrand {
    pub fn builder() -> SearchMediaResponseMediaItemBrandBuilder {
        <SearchMediaResponseMediaItemBrandBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMediaResponseMediaItemBrandBuilder {
    id: Option<String>,
    facebook_ad_library_id: Option<String>,
    name: Option<String>,
    r#type: Option<SearchMediaResponseMediaItemBrandType>,
    ig_username: Option<String>,
    ig_followers: Option<f64>,
    tiktok_username: Option<String>,
    tiktok_followers: Option<f64>,
    facebook_username: Option<String>,
    thumbnail_url: Option<String>,
    category: Option<String>,
}

impl SearchMediaResponseMediaItemBrandBuilder {
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

    pub fn r#type(mut self, value: SearchMediaResponseMediaItemBrandType) -> Self {
        self.r#type = Some(value);
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

    pub fn tiktok_username(mut self, value: impl Into<String>) -> Self {
        self.tiktok_username = Some(value.into());
        self
    }

    pub fn tiktok_followers(mut self, value: f64) -> Self {
        self.tiktok_followers = Some(value);
        self
    }

    pub fn facebook_username(mut self, value: impl Into<String>) -> Self {
        self.facebook_username = Some(value.into());
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchMediaResponseMediaItemBrand`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SearchMediaResponseMediaItemBrandBuilder::id)
    /// - [`name`](SearchMediaResponseMediaItemBrandBuilder::name)
    pub fn build(self) -> Result<SearchMediaResponseMediaItemBrand, BuildError> {
        Ok(SearchMediaResponseMediaItemBrand {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            facebook_ad_library_id: self.facebook_ad_library_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
            ig_username: self.ig_username,
            ig_followers: self.ig_followers,
            tiktok_username: self.tiktok_username,
            tiktok_followers: self.tiktok_followers,
            facebook_username: self.facebook_username,
            thumbnail_url: self.thumbnail_url,
            category: self.category,
        })
    }
}

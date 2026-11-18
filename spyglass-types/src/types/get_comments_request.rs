pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCommentsRequest {
    /// Instagram post shortcodes or post URLs
    #[serde(rename = "instagramIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_ids: Option<Vec<String>>,
    /// TikTok video ids or post URLs
    #[serde(rename = "tiktokIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_ids: Option<Vec<String>>,
    /// Comment pages to fetch per post. Default 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetCommentsRequest {
    pub fn builder() -> GetCommentsRequestBuilder {
        <GetCommentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCommentsRequestBuilder {
    instagram_ids: Option<Vec<String>>,
    tiktok_ids: Option<Vec<String>>,
    limit: Option<i64>,
}

impl GetCommentsRequestBuilder {
    pub fn instagram_ids(mut self, value: Vec<String>) -> Self {
        self.instagram_ids = Some(value);
        self
    }

    pub fn tiktok_ids(mut self, value: Vec<String>) -> Self {
        self.tiktok_ids = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCommentsRequest`].
    pub fn build(self) -> Result<GetCommentsRequest, BuildError> {
        Ok(GetCommentsRequest {
            instagram_ids: self.instagram_ids,
            tiktok_ids: self.tiktok_ids,
            limit: self.limit,
        })
    }
}


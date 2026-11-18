pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for posts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostsQueryRequest {
    /// Instagram handle. Exactly one of igUsername, tiktokUsername, or facebookAdLibraryId. A leading @ is ignored. sortBy is ignored.
    #[serde(rename = "igUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_username: Option<String>,
    /// TikTok handle. Exactly one identifier. sortBy is optional: most_recent (default) or popular. A leading @ is ignored. Do not send tagged.
    #[serde(rename = "tiktokUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_username: Option<String>,
    /// Facebook Ad Library page id. Exactly one identifier. sortBy is optional: most_recent (default) or most_impressions. Do not send tagged.
    #[serde(rename = "facebookAdLibraryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_ad_library_id: Option<String>,
    /// Only with igUsername. true returns posts the account is tagged in. Omit for the account’s own posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged: Option<PostsAccountRequestTagged>,
    /// Only with igUsername. posts (default) or reels. Cannot be combined with tagged. Rejected for other identifiers.
    #[serde(rename = "postType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_type: Option<PostsAccountRequestPostType>,
    /// true transcribes each video and returns transcript. Omit or false to skip transcription.
    #[serde(rename = "includeTranscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transcript: Option<PostsAccountRequestIncludeTranscript>,
    /// Optional. With tiktokUsername: most_recent (default) or popular. With facebookAdLibraryId: most_recent (default) or most_impressions. Ignored with igUsername.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<PostsAccountRequestSortBy>,
    /// Maximum posts to return. Default 20, max 120.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl PostsQueryRequest {
    pub fn builder() -> PostsQueryRequestBuilder {
        <PostsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostsQueryRequestBuilder {
    ig_username: Option<String>,
    tiktok_username: Option<String>,
    facebook_ad_library_id: Option<String>,
    tagged: Option<PostsAccountRequestTagged>,
    post_type: Option<PostsAccountRequestPostType>,
    include_transcript: Option<PostsAccountRequestIncludeTranscript>,
    sort_by: Option<PostsAccountRequestSortBy>,
    limit: Option<i64>,
}

impl PostsQueryRequestBuilder {
    pub fn ig_username(mut self, value: impl Into<String>) -> Self {
        self.ig_username = Some(value.into());
        self
    }

    pub fn tiktok_username(mut self, value: impl Into<String>) -> Self {
        self.tiktok_username = Some(value.into());
        self
    }

    pub fn facebook_ad_library_id(mut self, value: impl Into<String>) -> Self {
        self.facebook_ad_library_id = Some(value.into());
        self
    }

    pub fn tagged(mut self, value: PostsAccountRequestTagged) -> Self {
        self.tagged = Some(value);
        self
    }

    pub fn post_type(mut self, value: PostsAccountRequestPostType) -> Self {
        self.post_type = Some(value);
        self
    }

    pub fn include_transcript(mut self, value: PostsAccountRequestIncludeTranscript) -> Self {
        self.include_transcript = Some(value);
        self
    }

    pub fn sort_by(mut self, value: PostsAccountRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostsQueryRequest`].
    pub fn build(self) -> Result<PostsQueryRequest, BuildError> {
        Ok(PostsQueryRequest {
            ig_username: self.ig_username,
            tiktok_username: self.tiktok_username,
            facebook_ad_library_id: self.facebook_ad_library_id,
            tagged: self.tagged,
            post_type: self.post_type,
            include_transcript: self.include_transcript,
            sort_by: self.sort_by,
            limit: self.limit,
        })
    }
}


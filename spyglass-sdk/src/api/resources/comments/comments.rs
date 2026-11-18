use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CommentsClient {
    pub http_client: HttpClient,
}

impl CommentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return public comments for specific Instagram or TikTok posts.
    /// Pass post URLs or the platform id. At least one of `instagramIds` or
    /// `tiktokIds` is required (max 20 each). `limit` is comment pages
    /// per post (default 1, max 20).
    ///
    /// Instagram accepts a shortcode or a post URL:
    ///
    /// - `C2xY9abcD1e`
    /// - `https://www.instagram.com/p/C2xY9abcD1e/`
    /// - `https://www.instagram.com/reel/C2xY9abcD1e/`
    ///
    /// TikTok accepts a video id or a post URL:
    ///
    /// - `7234567890123456789`
    /// - `https://www.tiktok.com/@nike/video/7234567890123456789`
    ///
    /// Recent posts may be scraped live; older posts with stored comments
    /// are served from cache.
    ///
    /// Limited to 20 requests per minute per user.
    /// Costs $0.0050 per successful page of comments returned, charged against your plan's monthly credits.
    /// Each comment page fetched live (`limit`, default 1) is one billed
    /// unit. Cached posts do not scrape and are not billed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use spyglass_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SpyglassClient::new(config).expect("Failed to build client");
    ///     client
    ///         .comments
    ///         .get(
    ///             &GetCommentsRequest {
    ///                 instagram_ids: Some(vec!["https://www.instagram.com/p/C2xY9abcD1e/".to_string()]),
    ///                 tiktok_ids: Some(vec![
    ///                     "https://www.tiktok.com/@nike/video/7234567890123456789".to_string(),
    ///                 ]),
    ///                 limit: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &GetCommentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCommentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "comments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

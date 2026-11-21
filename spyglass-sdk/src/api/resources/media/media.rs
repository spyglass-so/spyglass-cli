use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct MediaClient {
    pub http_client: HttpClient,
}

impl MediaClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return media posts. A call usually takes under 10 seconds.
    ///
    /// **1. Pick what to search**
    ///
    /// | You want | Send |
    /// | --- | --- |
    /// | All media | Send neither `accountId` nor `query` to list media across the index, ordered by `sortBy` |
    /// | One account's media | `accountId` |
    /// | Media matching a description | `query`, which matches posts by what they show or say |
    /// | One account's media matching a description | `accountId` and `query` |
    /// | A broader category | `taxonomy`, a path from GET /taxonomies |
    ///
    /// **2. Pick a sort order**
    ///
    /// `sortBy` defaults to `MOST_RECENT`. Some orders only work with certain channels:
    ///
    /// | `sortBy` | Allowed `channel` |
    /// | --- | --- |
    /// | `MOST_RECENT`, `OLDEST` | Any |
    /// | `MOST_IMPRESSIONS`, `LONGEST_RUN` | Exactly `META_AD` |
    /// | `MOST_VIEWS`, `MOST_LIKES`, `MOST_COMMENTS`, `HIGHEST_ENGAGEMENT`, `HIGHEST_OUTLIER` | Anything except `META_AD` |
    ///
    /// **3. Optionally add a transcript or caption**
    ///
    /// Send `includeTranscript: true` to get each video transcript.
    /// Send `includeCaption: true` to get each post caption.
    /// Both default to false and do not change the price.
    ///
    /// **Cost and limits**
    ///
    /// Costs $0.0010 per creative returned, charged against your plan's monthly credits. The count field is what you were charged for.
    /// You are always charged for at least one row. If nothing matches, `count` is 0 but you pay for one row.
    /// Limited to 120 requests per minute per user.
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
    ///         .media
    ///         .search(
    ///             &SearchMediaRequest {
    ///                 query: Some("comfort-focused video ads".to_string()),
    ///                 limit: Some(20),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &SearchMediaRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchMediaResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "media",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

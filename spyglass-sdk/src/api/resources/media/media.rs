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

    /// Return creatives. Pass `accountId` to scope to one brand or creator,
    /// or omit it to search the whole corpus. Without `accountId`,
    /// `query` is required. A call typically takes under 10 seconds.
    /// Set `includeTranscript` to true to include each video transcript.
    /// That does not change the credit charge. The default is false.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per creative returned, charged against your plan's monthly credits. The count field is what you were charged for.
    /// The call bills a minimum of one row even when nothing matches. In that
    /// case `count` is 0 but you are charged for one row.
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

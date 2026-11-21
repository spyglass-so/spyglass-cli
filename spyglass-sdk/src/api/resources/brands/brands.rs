use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BrandsClient {
    pub http_client: HttpClient,
}

impl BrandsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Find brands.
    /// Filter by taxonomy, location, or followers.
    ///
    /// Leave the search fields empty to list by `sortBy` (default `MOST_FOLLOWERS`).
    /// Or pass one of:
    ///
    /// - `query` — semantic search. Ordered by relevance, so `sortBy` is ignored.
    /// Bills at least one row even when nothing matches.
    /// - `names` — name contains this text.
    /// - `ids` — UUID, handle, domain, or Ad Library id. Misses are in
    /// `unmatchedIds`. Ignores `limit` and `offset`.
    /// - `igUsername` or `tiktokUsername` — handle contains this text. A leading
    /// `@` is ignored.
    ///
    /// When more than one is set:
    ///
    /// `ids` > `igUsername` / `tiktokUsername` > `names` > `query`
    ///
    /// `names` + `query` uses both.
    ///
    /// `limit` is 20 by default, 100 max, and caps the charge except for `ids`.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per brand returned, charged against your plan's monthly credits. The count field is what you were charged for.
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
    ///         .brands
    ///         .search(
    ///             &SearchBrandsRequest {
    ///                 names: Some(vec!["Nike".to_string()]),
    ///                 limit: Some(3),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &SearchBrandsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchBrandsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "brands",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

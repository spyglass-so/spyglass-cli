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
    /// Search semantically with `query`, or look them up by name, id, or
    /// social handle. Either way, you can also narrow the results with
    /// filters such as product taxonomy, location, and follower count.
    ///
    /// Send one search field. Each field is its own mode:
    ///
    /// - **Query** (`query`). A natural-language brief. Runs a language model and
    /// ranks by relevance, so `sortBy` is ignored. Bills at least one row even
    /// when nothing matches.
    /// - **Name** (`names`). Fuzzy-matches account names (a case-insensitive contains
    /// match). Pass every name in one array.
    /// - **ID** (`ids`). Looks up UUIDs, Instagram handles, TikTok handles, website
    /// domains, or Facebook Ad Library ids. A leading `@` is ignored. Missing ids are omitted and listed
    /// in `unmatchedIds`. `limit` and `offset` are ignored.
    /// - **Instagram** (`igUsername`). Case-insensitive contains match on the
    /// handle. A leading `@` is ignored.
    /// - **TikTok** (`tiktokUsername`). Same match on the TikTok handle.
    ///
    /// If a request includes more than one of these fields, only the highest
    /// priority field is used. The rest are ignored:
    ///
    /// 1. `ids`
    /// 2. `igUsername` or `tiktokUsername` (if both are set, either handle can match)
    /// 3. `names`
    /// 4. `query`
    ///
    /// One exception: `query` together with `names` uses both. `names` narrows the
    /// set, and `query` ranks inside it.
    ///
    /// `limit` (default 20, max 100) caps the rows returned, and the cost, for
    /// every mode except ID.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per brand returned, charged against your plan's monthly credits. The count field is what you were charged for.
    /// A `query` search runs a language model, so it bills a minimum of one
    /// row even when nothing matches.
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

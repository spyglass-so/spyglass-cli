use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TaxonomiesClient {
    pub http_client: HttpClient,
}

impl TaxonomiesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Find taxonomy paths you can pass to POST /media as `taxonomy`.
    /// `query` is a product or category phrase. Each path is a string
    /// such as `Apparel > Footwear > Sneakers`.
    ///
    /// Limited to 120 requests per minute per user.
    /// This call is free and does not use credits.
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
    ///         .taxonomies
    ///         .search(
    ///             &SearchQueryRequest {
    ///                 query: "running shoes".to_string(),
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &SearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchTaxonomiesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "taxonomies",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}

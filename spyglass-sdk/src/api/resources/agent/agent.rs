use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct AgentClient {
    pub http_client: HttpClient,
}

impl AgentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists runs for this team, newest first.
    /// Page with `offset`. The next page is `offset` plus the `limit` you sent.
    /// `count` is the number of runs in this response. Stop when `count` is less than `limit`.
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
    ///         .agent
    ///         .list(
    ///             &ListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "agent/runs",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Start a Spyglass agent run. The run id is the chat thread id.
    /// Pass `outputSchema` to get that JSON on `structured` when the run completes.
    /// The root `type` must be `"object"`. Each property can be a string, number, integer, boolean, array, or object.
    /// `model` is `default` or `pro`.
    /// Poll GET /agent/runs/{id} until `status` is `completed` or `failed`.
    ///
    /// Limited to 20 requests per minute per user.
    /// The create call is free. The run is billed from model usage when it finishes.
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
    ///         .agent
    ///         .create(
    ///             &CreateAgentRequest {
    ///                 query: "query".to_string(),
    ///                 output_schema: None,
    ///                 system_prompt: None,
    ///                 effort: None,
    ///                 model: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "agent/runs",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Poll a run until `status` is `completed` or `failed`.
    /// Completed runs include `text` and, when you passed a schema, `structured`.
    /// Failed runs include `error` with a `code` and `message`.
    ///
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
    ///     client.agent.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("agent/runs/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}

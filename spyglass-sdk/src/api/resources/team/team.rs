use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TeamClient {
    pub http_client: HttpClient,
}

impl TeamClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Credits left on the API key's team, the credits included in the
    /// plan, and the current billing period. Granted credits count toward
    /// `remainingCredits` and not `planCredits`.
    ///
    /// Limited to 60 requests per minute per user.
    /// This call is free and does not consume credits. It still responds when
    /// the team is out of credits.
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
    ///     client.team.credit_usage(None).await;
    /// }
    /// ```
    pub async fn credit_usage(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<CreditUsageTeamResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "team/credit-usage", None, None, options)
            .await
    }
}

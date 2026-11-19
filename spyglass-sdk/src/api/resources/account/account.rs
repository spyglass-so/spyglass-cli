use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AccountClient {
    pub http_client: HttpClient,
}

impl AccountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Fetch a single brand or creator. `id` is any one of:
    ///
    /// - an account id (a UUID, or the numeric id from POST /brands or POST /creators)
    /// - a website URL or bare domain, such as `https://www.nike.com` or `nike.com`
    /// - an Instagram handle, such as `nike` or `@nike`
    /// - a TikTok handle, such as `nike` or `@nike`
    ///
    /// A website matches the account domain. A handle matches `igUsername` or
    /// `tiktokUsername`. When both match, the `igUsername` match is returned.
    /// Encode `/` and `?` in a URL. The `type` field tells you which
    /// one you got: `BRAND` or `CREATOR`, and the creator-only fields
    /// (`gender`, `ageRange`, ...) are present only on `CREATOR`.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per account returned, charged against your plan's monthly credits.
    /// An id that matches nothing returns 404 and is free.
    ///
    /// # Arguments
    ///
    /// * `id` - Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL.
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
    ///         .account
    ///         .get(&"e9cf96ee-7455-4b00-b692-80e2f923f79d".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetAccountResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("account/{}", id), None, None, options)
            .await
    }

    /// Return creative insights for an account. `MEDIA_MIX` counts creatives by
    /// media type, `CATEGORY` by creative category, and `FUNNEL` by funnel
    /// stage. `HOOK`, `TARGET_PERSONA`, and `AD_ANGLE` return ranked totals
    /// (`id`, `label`, `tags`, `total`), highest first.
    ///
    /// If a hook, angle, or persona clustering run is missing, older than 30
    /// days, or has no clusters, clustering starts and the response includes
    /// `status: "analyzing"` with empty totals. Retry for the finished clusters.
    /// Analyzing responses are not charged.
    ///
    /// `datePreset`, or `startDate` and `endDate` as `YYYY-MM-DD`, filter
    /// by creative start date. Omit both for the last 30 days.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per cluster returned, charged against your plan's monthly credits. The count field is what you were charged for.
    ///
    /// # Arguments
    ///
    /// * `id` - Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL.
    /// * `start_date` - Calendar date, YYYY-MM-DD.
    /// * `end_date` - Calendar date, YYYY-MM-DD.
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
    ///         .account
    ///         .insights(
    ///             &"e9cf96ee-7455-4b00-b692-80e2f923f79d".to_string(),
    ///             &InsightsQueryRequest {
    ///                 r#type: InsightsAccountRequestType::Hook,
    ///                 date_preset: None,
    ///                 start_date: None,
    ///                 end_date: None,
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn insights(
        &self,
        id: &str,
        request: &InsightsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InsightsAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("account/{}/insights", id),
                None,
                QueryBuilder::new()
                    .serialize("type", Some(request.r#type.clone()))
                    .serialize("datePreset", request.date_preset.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Return an account’s campaigns or offers, newest start date first.
    /// `datePreset`, or `startDate` and `endDate` as `YYYY-MM-DD`, filter
    /// by campaign start date. Omit both for all time.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per campaign returned, charged against your plan's monthly credits. The count field is what you were charged for.
    ///
    /// # Arguments
    ///
    /// * `id` - Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL.
    /// * `start_date` - Calendar date, YYYY-MM-DD.
    /// * `end_date` - Calendar date, YYYY-MM-DD.
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
    ///         .account
    ///         .campaigns(
    ///             &"e9cf96ee-7455-4b00-b692-80e2f923f79d".to_string(),
    ///             &CampaignsQueryRequest {
    ///                 r#type: CampaignsAccountRequestType::Campaign,
    ///                 sort_by: None,
    ///                 limit: None,
    ///                 date_preset: None,
    ///                 start_date: None,
    ///                 end_date: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn campaigns(
        &self,
        id: &str,
        request: &CampaignsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignsAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("account/{}/campaigns", id),
                None,
                QueryBuilder::new()
                    .serialize("type", Some(request.r#type.clone()))
                    .serialize("sortBy", request.sort_by.clone())
                    .int("limit", request.limit.clone())
                    .serialize("datePreset", request.date_preset.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Return the creators an account has collaborated with, ordered from
    /// most media to least. Works for both brands and creators.
    /// mediaCount is how many of that account’s media items tag the
    /// creator. startedAt is the earliest of those media start times, and
    /// lastPostedAt is the latest. `datePreset`, or `startDate` and
    /// `endDate` as `YYYY-MM-DD`, filter by startedAt. Omit both for all time.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per collab returned, charged against your plan's monthly credits. The count field is what you were charged for.
    ///
    /// # Arguments
    ///
    /// * `id` - Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL.
    /// * `start_date` - Calendar date, YYYY-MM-DD.
    /// * `end_date` - Calendar date, YYYY-MM-DD.
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
    ///         .account
    ///         .collabs(
    ///             &"e9cf96ee-7455-4b00-b692-80e2f923f79d".to_string(),
    ///             &CollabsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn collabs(
        &self,
        id: &str,
        request: &CollabsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CollabsAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("account/{}/collabs", id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("datePreset", request.date_preset.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Return the landing-page URLs an account sends ad traffic to, ranked by
    /// how often they appear across its ads.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per landing page returned, charged against your plan's monthly credits. The count field is what you were charged for.
    ///
    /// # Arguments
    ///
    /// * `id` - Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL.
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
    ///         .account
    ///         .landing_pages(
    ///             &"e9cf96ee-7455-4b00-b692-80e2f923f79d".to_string(),
    ///             &LandingPagesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn landing_pages(
        &self,
        id: &str,
        request: &LandingPagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandingPagesAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("account/{}/landing-pages", id),
                None,
                QueryBuilder::new()
                    .serialize("showInactive", request.show_inactive.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Return one account's posts, fetched live. The account does not need to be indexed.
    /// A call usually takes 10–30 seconds and starts indexing the rest of the brand's posts.
    /// Use `POST /media` first when you already have an account id. See the Finding media guide.
    ///
    /// Limited to 120 requests per minute per user.
    /// Costs $0.0010 per creative returned, charged against your plan's monthly credits. The count field is what you were charged for.
    /// The call bills a minimum of one row even when nothing matches. In that
    /// case `count` is 0 but you are charged for one row.
    ///
    /// Send exactly one identifier. The fields you can send depend on which
    /// one you pick.
    ///
    /// **`igUsername`**
    ///
    /// | Field | Details |
    /// | --- | --- |
    /// | `igUsername` | Instagram handle. A leading `@` is ignored. |
    /// | `tagged` | Optional. `true` returns posts the account is tagged in. Omit it for the account's own posts. Cannot be combined with `postType=reels`. |
    /// | `postType` | Optional, Instagram only. `posts` (default) or `reels`. Cannot be combined with `tagged`. |
    /// | `includeTranscript` | Optional. `true` transcribes each video and returns `transcript`. Omit it or send `false` to skip transcription. The call takes longer. |
    /// | `limit` | Optional. Default 20, max 120. |
    /// | `sortBy` | Ignored. Posts are always most recent. |
    ///
    /// **`tiktokUsername`**
    ///
    /// | Field | Details |
    /// | --- | --- |
    /// | `tiktokUsername` | TikTok handle. A leading `@` is ignored. |
    /// | `includeTranscript` | Optional. `true` transcribes each video and returns `transcript`. Omit it or send `false` to skip transcription. The call takes longer. |
    /// | `sortBy` | Optional. `most_recent` (default) or `popular`. |
    /// | `limit` | Optional. Default 20, max 120. |
    ///
    /// **`facebookAdLibraryId`**
    ///
    /// | Field | Details |
    /// | --- | --- |
    /// | `facebookAdLibraryId` | Facebook Ad Library page id. |
    /// | `includeTranscript` | Optional. `true` transcribes each video and returns `transcript`. Omit it or send `false` to skip transcription. The call takes longer. |
    /// | `sortBy` | Optional. `most_recent` (default) or `most_impressions`. |
    /// | `limit` | Optional. Default 20, max 120. |
    ///
    /// # Arguments
    ///
    /// * `ig_username` - Instagram handle. Exactly one of igUsername, tiktokUsername, or facebookAdLibraryId. A leading @ is ignored. sortBy is ignored.
    /// * `tiktok_username` - TikTok handle. Exactly one identifier. sortBy is optional: most_recent (default) or popular. A leading @ is ignored. Do not send tagged.
    /// * `facebook_ad_library_id` - Facebook Ad Library page id. Exactly one identifier. sortBy is optional: most_recent (default) or most_impressions. Do not send tagged.
    /// * `tagged` - Only with igUsername. true returns posts the account is tagged in. Omit for the account’s own posts.
    /// * `post_type` - Only with igUsername. posts (default) or reels. Cannot be combined with tagged. Rejected for other identifiers.
    /// * `include_transcript` - true transcribes each video and returns transcript. Omit or false to skip transcription.
    /// * `sort_by` - Optional. With tiktokUsername: most_recent (default) or popular. With facebookAdLibraryId: most_recent (default) or most_impressions. Ignored with igUsername.
    /// * `limit` - Maximum posts to return. Default 20, max 120.
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
    ///         .account
    ///         .posts(
    ///             &PostsQueryRequest {
    ///                 ig_username: Some("nike".to_string()),
    ///                 limit: Some(20),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn posts(
        &self,
        request: &PostsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostsAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "account/posts",
                None,
                QueryBuilder::new()
                    .string("igUsername", request.ig_username.clone())
                    .string("tiktokUsername", request.tiktok_username.clone())
                    .string(
                        "facebookAdLibraryId",
                        request.facebook_ad_library_id.clone(),
                    )
                    .serialize("tagged", request.tagged.clone())
                    .serialize("postType", request.post_type.clone())
                    .serialize("includeTranscript", request.include_transcript.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Look up a brand by website URL, or start indexing it if we do not
    /// have it yet. Pass a single `websiteUrl`. Instagram and TikTok
    /// profile URLs are accepted and resolved as creators.
    ///
    /// An existing brand returns
    /// `status: "already_exists"`. A new brand is created and sync is
    /// queued, returning `status: "indexing"` and the new `brandId`.
    ///
    /// Limited to 50 requests per minute per user.
    /// This call is free and does not consume credits.
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
    ///         .account
    ///         .index(
    ///             &IndexAccountRequest {
    ///                 website_url: "https://www.nike.com".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn index(
        &self,
        request: &IndexAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<IndexAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "account/index",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Poll whether a brand finished indexing after POST /account/index.
    ///
    /// Limited to 300 requests per minute per user.
    /// This call is free and does not consume credits.
    ///
    /// # Arguments
    ///
    /// * `id` - Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL.
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
    ///         .account
    ///         .index_status(&"e9cf96ee-7455-4b00-b692-80e2f923f79d".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn index_status(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<IndexStatusAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("account/{}/index-status", id),
                None,
                None,
                options,
            )
            .await
    }
}

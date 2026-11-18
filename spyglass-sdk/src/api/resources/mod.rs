//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Brands**
//! - **Creators**
//! - **Account**
//! - **Taxonomies**
//! - **Media**
//! - **Team**
//! - **Comments**
//! - **Agent**

use crate::{ApiError, ClientConfig};

pub mod account;
pub mod agent;
pub mod brands;
pub mod comments;
pub mod creators;
pub mod media;
pub mod taxonomies;
pub mod team;
pub struct ApiClient {
    pub config: ClientConfig,
    pub brands: BrandsClient,
    pub creators: CreatorsClient,
    pub account: AccountClient,
    pub taxonomies: TaxonomiesClient,
    pub media: MediaClient,
    pub team: TeamClient,
    pub comments: CommentsClient,
    pub agent: AgentClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            brands: BrandsClient::new(config.clone())?,
            creators: CreatorsClient::new(config.clone())?,
            account: AccountClient::new(config.clone())?,
            taxonomies: TaxonomiesClient::new(config.clone())?,
            media: MediaClient::new(config.clone())?,
            team: TeamClient::new(config.clone())?,
            comments: CommentsClient::new(config.clone())?,
            agent: AgentClient::new(config.clone())?,
        })
    }
}

pub use account::AccountClient;
pub use agent::AgentClient;
pub use brands::BrandsClient;
pub use comments::CommentsClient;
pub use creators::CreatorsClient;
pub use media::MediaClient;
pub use taxonomies::TaxonomiesClient;
pub use team::TeamClient;

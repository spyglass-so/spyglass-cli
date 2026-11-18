//! API client and types for the Spyglass API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    AccountClient, AgentClient, ApiClient, BrandsClient, CommentsClient, CreatorsClient,
    MediaClient, TaxonomiesClient, TeamClient,
};

pub use spyglass_types::*;

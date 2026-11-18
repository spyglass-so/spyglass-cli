pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for landing-pages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPagesQueryRequest {
    #[serde(rename = "showInactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_inactive: Option<LandingPagesAccountRequestShowInactive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl LandingPagesQueryRequest {
    pub fn builder() -> LandingPagesQueryRequestBuilder {
        <LandingPagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPagesQueryRequestBuilder {
    show_inactive: Option<LandingPagesAccountRequestShowInactive>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl LandingPagesQueryRequestBuilder {
    pub fn show_inactive(mut self, value: LandingPagesAccountRequestShowInactive) -> Self {
        self.show_inactive = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPagesQueryRequest`].
    pub fn build(self) -> Result<LandingPagesQueryRequest, BuildError> {
        Ok(LandingPagesQueryRequest {
            show_inactive: self.show_inactive,
            limit: self.limit,
            offset: self.offset,
        })
    }
}


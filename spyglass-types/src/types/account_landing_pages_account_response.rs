pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LandingPagesAccountResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(rename = "landingPages")]
    #[serde(default)]
    pub landing_pages: Vec<LandingPagesAccountResponseLandingPagesItem>,
}

impl LandingPagesAccountResponse {
    pub fn builder() -> LandingPagesAccountResponseBuilder {
        <LandingPagesAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPagesAccountResponseBuilder {
    count: Option<i64>,
    landing_pages: Option<Vec<LandingPagesAccountResponseLandingPagesItem>>,
}

impl LandingPagesAccountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn landing_pages(mut self, value: Vec<LandingPagesAccountResponseLandingPagesItem>) -> Self {
        self.landing_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPagesAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](LandingPagesAccountResponseBuilder::count)
    /// - [`landing_pages`](LandingPagesAccountResponseBuilder::landing_pages)
    pub fn build(self) -> Result<LandingPagesAccountResponse, BuildError> {
        Ok(LandingPagesAccountResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            landing_pages: self.landing_pages.ok_or_else(|| BuildError::missing_field("landing_pages"))?,
        })
    }
}

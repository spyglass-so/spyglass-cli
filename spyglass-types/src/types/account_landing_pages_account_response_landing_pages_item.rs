pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LandingPagesAccountResponseLandingPagesItem {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
}

impl LandingPagesAccountResponseLandingPagesItem {
    pub fn builder() -> LandingPagesAccountResponseLandingPagesItemBuilder {
        <LandingPagesAccountResponseLandingPagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPagesAccountResponseLandingPagesItemBuilder {
    url: Option<String>,
    total: Option<f64>,
}

impl LandingPagesAccountResponseLandingPagesItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPagesAccountResponseLandingPagesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](LandingPagesAccountResponseLandingPagesItemBuilder::url)
    /// - [`total`](LandingPagesAccountResponseLandingPagesItemBuilder::total)
    pub fn build(self) -> Result<LandingPagesAccountResponseLandingPagesItem, BuildError> {
        Ok(LandingPagesAccountResponseLandingPagesItem {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}

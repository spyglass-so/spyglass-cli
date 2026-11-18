pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InsightsAccountResponseStatusTotalsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
}

impl InsightsAccountResponseStatusTotalsItem {
    pub fn builder() -> InsightsAccountResponseStatusTotalsItemBuilder {
        <InsightsAccountResponseStatusTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsightsAccountResponseStatusTotalsItemBuilder {
    id: Option<String>,
    label: Option<String>,
    tags: Option<Vec<String>>,
    total: Option<f64>,
}

impl InsightsAccountResponseStatusTotalsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsightsAccountResponseStatusTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InsightsAccountResponseStatusTotalsItemBuilder::id)
    /// - [`label`](InsightsAccountResponseStatusTotalsItemBuilder::label)
    /// - [`total`](InsightsAccountResponseStatusTotalsItemBuilder::total)
    pub fn build(self) -> Result<InsightsAccountResponseStatusTotalsItem, BuildError> {
        Ok(InsightsAccountResponseStatusTotalsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            tags: self.tags,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}

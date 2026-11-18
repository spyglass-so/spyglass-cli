pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignsAccountResponseCampaignsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "velocityScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub velocity_score: Option<f64>,
    #[serde(rename = "startAt")]
    #[serde(default)]
    pub start_at: String,
    #[serde(rename = "mediaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub media_count: Option<f64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub launches7d: f64,
}

impl CampaignsAccountResponseCampaignsItem {
    pub fn builder() -> CampaignsAccountResponseCampaignsItemBuilder {
        <CampaignsAccountResponseCampaignsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsAccountResponseCampaignsItemBuilder {
    id: Option<String>,
    r#type: Option<String>,
    category: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    velocity_score: Option<f64>,
    start_at: Option<String>,
    media_count: Option<f64>,
    launches7d: Option<f64>,
}

impl CampaignsAccountResponseCampaignsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn velocity_score(mut self, value: f64) -> Self {
        self.velocity_score = Some(value);
        self
    }

    pub fn start_at(mut self, value: impl Into<String>) -> Self {
        self.start_at = Some(value.into());
        self
    }

    pub fn media_count(mut self, value: f64) -> Self {
        self.media_count = Some(value);
        self
    }

    pub fn launches7d(mut self, value: f64) -> Self {
        self.launches7d = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsAccountResponseCampaignsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CampaignsAccountResponseCampaignsItemBuilder::id)
    /// - [`r#type`](CampaignsAccountResponseCampaignsItemBuilder::r#type)
    /// - [`title`](CampaignsAccountResponseCampaignsItemBuilder::title)
    /// - [`start_at`](CampaignsAccountResponseCampaignsItemBuilder::start_at)
    /// - [`launches7d`](CampaignsAccountResponseCampaignsItemBuilder::launches7d)
    pub fn build(self) -> Result<CampaignsAccountResponseCampaignsItem, BuildError> {
        Ok(CampaignsAccountResponseCampaignsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            category: self.category,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            summary: self.summary,
            velocity_score: self.velocity_score,
            start_at: self.start_at.ok_or_else(|| BuildError::missing_field("start_at"))?,
            media_count: self.media_count,
            launches7d: self.launches7d.ok_or_else(|| BuildError::missing_field("launches7d"))?,
        })
    }
}

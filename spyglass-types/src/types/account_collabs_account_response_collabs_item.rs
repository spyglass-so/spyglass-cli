pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabsAccountResponseCollabsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "profileUrl")]
    #[serde(default)]
    pub profile_url: String,
    #[serde(rename = "mediaCount")]
    #[serde(default)]
    pub media_count: i64,
    /// When the collab started: the earliest media start time.
    #[serde(rename = "startedAt")]
    #[serde(default)]
    pub started_at: String,
    /// The latest media post time for this creator.
    #[serde(rename = "lastPostedAt")]
    #[serde(default)]
    pub last_posted_at: String,
}

impl CollabsAccountResponseCollabsItem {
    pub fn builder() -> CollabsAccountResponseCollabsItemBuilder {
        <CollabsAccountResponseCollabsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabsAccountResponseCollabsItemBuilder {
    name: Option<String>,
    profile_url: Option<String>,
    media_count: Option<i64>,
    started_at: Option<String>,
    last_posted_at: Option<String>,
}

impl CollabsAccountResponseCollabsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_url(mut self, value: impl Into<String>) -> Self {
        self.profile_url = Some(value.into());
        self
    }

    pub fn media_count(mut self, value: i64) -> Self {
        self.media_count = Some(value);
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn last_posted_at(mut self, value: impl Into<String>) -> Self {
        self.last_posted_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CollabsAccountResponseCollabsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CollabsAccountResponseCollabsItemBuilder::name)
    /// - [`profile_url`](CollabsAccountResponseCollabsItemBuilder::profile_url)
    /// - [`media_count`](CollabsAccountResponseCollabsItemBuilder::media_count)
    /// - [`started_at`](CollabsAccountResponseCollabsItemBuilder::started_at)
    /// - [`last_posted_at`](CollabsAccountResponseCollabsItemBuilder::last_posted_at)
    pub fn build(self) -> Result<CollabsAccountResponseCollabsItem, BuildError> {
        Ok(CollabsAccountResponseCollabsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            profile_url: self.profile_url.ok_or_else(|| BuildError::missing_field("profile_url"))?,
            media_count: self.media_count.ok_or_else(|| BuildError::missing_field("media_count"))?,
            started_at: self.started_at.ok_or_else(|| BuildError::missing_field("started_at"))?,
            last_posted_at: self.last_posted_at.ok_or_else(|| BuildError::missing_field("last_posted_at"))?,
        })
    }
}

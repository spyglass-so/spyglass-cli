pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAgentResponse {
    #[serde(default)]
    pub id: String,
    pub status: CreateAgentResponseStatus,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl CreateAgentResponse {
    pub fn builder() -> CreateAgentResponseBuilder {
        <CreateAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentResponseBuilder {
    id: Option<String>,
    status: Option<CreateAgentResponseStatus>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl CreateAgentResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateAgentResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateAgentResponseBuilder::id)
    /// - [`status`](CreateAgentResponseBuilder::status)
    /// - [`created_at`](CreateAgentResponseBuilder::created_at)
    pub fn build(self) -> Result<CreateAgentResponse, BuildError> {
        Ok(CreateAgentResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}

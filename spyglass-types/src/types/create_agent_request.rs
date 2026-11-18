pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAgentRequest {
    #[serde(default)]
    pub query: String,
    /// JSON Schema for `structured`. The root `type` must be `"object"`. Each property can be a string, number, integer, boolean, array, or object.
    #[serde(rename = "outputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<CreateAgentRequestOutputSchema>,
    #[serde(rename = "systemPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<CreateAgentRequestEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<CreateAgentRequestModel>,
}

impl CreateAgentRequest {
    pub fn builder() -> CreateAgentRequestBuilder {
        <CreateAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRequestBuilder {
    query: Option<String>,
    output_schema: Option<CreateAgentRequestOutputSchema>,
    system_prompt: Option<String>,
    effort: Option<CreateAgentRequestEffort>,
    model: Option<CreateAgentRequestModel>,
}

impl CreateAgentRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn output_schema(mut self, value: CreateAgentRequestOutputSchema) -> Self {
        self.output_schema = Some(value);
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn effort(mut self, value: CreateAgentRequestEffort) -> Self {
        self.effort = Some(value);
        self
    }

    pub fn model(mut self, value: CreateAgentRequestModel) -> Self {
        self.model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](CreateAgentRequestBuilder::query)
    pub fn build(self) -> Result<CreateAgentRequest, BuildError> {
        Ok(CreateAgentRequest {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            output_schema: self.output_schema,
            system_prompt: self.system_prompt,
            effort: self.effort,
            model: self.model,
        })
    }
}


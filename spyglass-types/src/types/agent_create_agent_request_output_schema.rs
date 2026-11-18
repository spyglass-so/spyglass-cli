pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// JSON Schema for `structured`. The root `type` must be `"object"`. Each property can be a string, number, integer, boolean, array, or object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAgentRequestOutputSchema {
    pub r#type: CreateAgentRequestOutputSchemaType,
    /// Fields of the structured result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, CreateAgentRequestOutputSchemaPropertiesValue>>,
    /// Property names the result must include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl CreateAgentRequestOutputSchema {
    pub fn builder() -> CreateAgentRequestOutputSchemaBuilder {
        <CreateAgentRequestOutputSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRequestOutputSchemaBuilder {
    r#type: Option<CreateAgentRequestOutputSchemaType>,
    properties: Option<HashMap<String, CreateAgentRequestOutputSchemaPropertiesValue>>,
    required: Option<Vec<String>>,
}

impl CreateAgentRequestOutputSchemaBuilder {
    pub fn r#type(mut self, value: CreateAgentRequestOutputSchemaType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, CreateAgentRequestOutputSchemaPropertiesValue>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentRequestOutputSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CreateAgentRequestOutputSchemaBuilder::r#type)
    pub fn build(self) -> Result<CreateAgentRequestOutputSchema, BuildError> {
        Ok(CreateAgentRequestOutputSchema {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            properties: self.properties,
            required: self.required,
        })
    }
}

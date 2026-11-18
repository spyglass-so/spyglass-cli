pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// JSON Schema for one field. Nested objects and arrays use this same shape.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAgentRequestOutputSchemaPropertiesValue {
    pub r#type: CreateAgentRequestOutputSchemaPropertiesValueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Item schema when `type` is `array`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<HashMap<String, serde_json::Value>>,
    /// Nested fields when `type` is `object`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl CreateAgentRequestOutputSchemaPropertiesValue {
    pub fn builder() -> CreateAgentRequestOutputSchemaPropertiesValueBuilder {
        <CreateAgentRequestOutputSchemaPropertiesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRequestOutputSchemaPropertiesValueBuilder {
    r#type: Option<CreateAgentRequestOutputSchemaPropertiesValueType>,
    description: Option<String>,
    items: Option<HashMap<String, serde_json::Value>>,
    properties: Option<HashMap<String, serde_json::Value>>,
    required: Option<Vec<String>>,
}

impl CreateAgentRequestOutputSchemaPropertiesValueBuilder {
    pub fn r#type(mut self, value: CreateAgentRequestOutputSchemaPropertiesValueType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn items(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentRequestOutputSchemaPropertiesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CreateAgentRequestOutputSchemaPropertiesValueBuilder::r#type)
    pub fn build(self) -> Result<CreateAgentRequestOutputSchemaPropertiesValue, BuildError> {
        Ok(CreateAgentRequestOutputSchemaPropertiesValue {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self.description,
            items: self.items,
            properties: self.properties,
            required: self.required,
        })
    }
}

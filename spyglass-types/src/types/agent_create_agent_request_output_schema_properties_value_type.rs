pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAgentRequestOutputSchemaPropertiesValueType {
    String_,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAgentRequestOutputSchemaPropertiesValueType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::String_ => serializer.serialize_str("string"),
            Self::Number => serializer.serialize_str("number"),
            Self::Integer => serializer.serialize_str("integer"),
            Self::Boolean => serializer.serialize_str("boolean"),
            Self::Array => serializer.serialize_str("array"),
            Self::Object => serializer.serialize_str("object"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAgentRequestOutputSchemaPropertiesValueType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "string" => Ok(Self::String_),
            "number" => Ok(Self::Number),
            "integer" => Ok(Self::Integer),
            "boolean" => Ok(Self::Boolean),
            "array" => Ok(Self::Array),
            "object" => Ok(Self::Object),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAgentRequestOutputSchemaPropertiesValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String_ => write!(f, "string"),
            Self::Number => write!(f, "number"),
            Self::Integer => write!(f, "integer"),
            Self::Boolean => write!(f, "boolean"),
            Self::Array => write!(f, "array"),
            Self::Object => write!(f, "object"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

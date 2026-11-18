pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostsAccountResponseMediaItemBrandType {
    Brand,
    Creator,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostsAccountResponseMediaItemBrandType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Brand => serializer.serialize_str("BRAND"),
            Self::Creator => serializer.serialize_str("CREATOR"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostsAccountResponseMediaItemBrandType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "BRAND" => Ok(Self::Brand),
            "CREATOR" => Ok(Self::Creator),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostsAccountResponseMediaItemBrandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Brand => write!(f, "BRAND"),
            Self::Creator => write!(f, "CREATOR"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

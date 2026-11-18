pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostsAccountResponseMediaItemMediaType {
    Video,
    Image,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostsAccountResponseMediaItemMediaType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Video => serializer.serialize_str("VIDEO"),
            Self::Image => serializer.serialize_str("IMAGE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostsAccountResponseMediaItemMediaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "VIDEO" => Ok(Self::Video),
            "IMAGE" => Ok(Self::Image),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostsAccountResponseMediaItemMediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Video => write!(f, "VIDEO"),
            Self::Image => write!(f, "IMAGE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

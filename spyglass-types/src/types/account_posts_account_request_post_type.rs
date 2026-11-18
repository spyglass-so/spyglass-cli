pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Only with igUsername. posts (default) or reels. Cannot be combined with tagged. Rejected for other identifiers.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostsAccountRequestPostType {
    Posts,
    Reels,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostsAccountRequestPostType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Posts => serializer.serialize_str("posts"),
            Self::Reels => serializer.serialize_str("reels"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostsAccountRequestPostType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "posts" => Ok(Self::Posts),
            "reels" => Ok(Self::Reels),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostsAccountRequestPostType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Posts => write!(f, "posts"),
            Self::Reels => write!(f, "reels"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

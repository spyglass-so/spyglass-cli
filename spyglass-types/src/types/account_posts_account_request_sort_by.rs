pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Optional. With tiktokUsername: most_recent (default) or popular. With facebookAdLibraryId: most_recent (default) or most_impressions. Ignored with igUsername.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostsAccountRequestSortBy {
    Popular,
    MostImpressions,
    MostRecent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostsAccountRequestSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Popular => serializer.serialize_str("popular"),
            Self::MostImpressions => serializer.serialize_str("most_impressions"),
            Self::MostRecent => serializer.serialize_str("most_recent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostsAccountRequestSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "popular" => Ok(Self::Popular),
            "most_impressions" => Ok(Self::MostImpressions),
            "most_recent" => Ok(Self::MostRecent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostsAccountRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Popular => write!(f, "popular"),
            Self::MostImpressions => write!(f, "most_impressions"),
            Self::MostRecent => write!(f, "most_recent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

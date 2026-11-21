pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostsAccountResponseMediaItemPlacementItem {
    Facebook,
    Instagram,
    AudienceNetwork,
    Messenger,
    Threads,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostsAccountResponseMediaItemPlacementItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Facebook => serializer.serialize_str("FACEBOOK"),
            Self::Instagram => serializer.serialize_str("INSTAGRAM"),
            Self::AudienceNetwork => serializer.serialize_str("AUDIENCE_NETWORK"),
            Self::Messenger => serializer.serialize_str("MESSENGER"),
            Self::Threads => serializer.serialize_str("THREADS"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostsAccountResponseMediaItemPlacementItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "FACEBOOK" => Ok(Self::Facebook),
            "INSTAGRAM" => Ok(Self::Instagram),
            "AUDIENCE_NETWORK" => Ok(Self::AudienceNetwork),
            "MESSENGER" => Ok(Self::Messenger),
            "THREADS" => Ok(Self::Threads),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostsAccountResponseMediaItemPlacementItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Facebook => write!(f, "FACEBOOK"),
            Self::Instagram => write!(f, "INSTAGRAM"),
            Self::AudienceNetwork => write!(f, "AUDIENCE_NETWORK"),
            Self::Messenger => write!(f, "MESSENGER"),
            Self::Threads => write!(f, "THREADS"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

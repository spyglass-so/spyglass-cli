pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaRequestChannelItem {
    MetaAd,
    Instagram,
    Tiktok,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaRequestChannelItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MetaAd => serializer.serialize_str("META_AD"),
            Self::Instagram => serializer.serialize_str("INSTAGRAM"),
            Self::Tiktok => serializer.serialize_str("TIKTOK"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaRequestChannelItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "META_AD" => Ok(Self::MetaAd),
            "INSTAGRAM" => Ok(Self::Instagram),
            "TIKTOK" => Ok(Self::Tiktok),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaRequestChannelItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaAd => write!(f, "META_AD"),
            Self::Instagram => write!(f, "INSTAGRAM"),
            Self::Tiktok => write!(f, "TIKTOK"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

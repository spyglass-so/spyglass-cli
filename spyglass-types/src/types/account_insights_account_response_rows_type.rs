pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InsightsAccountResponseRowsType {
    MediaMix,
    Category,
    Funnel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InsightsAccountResponseRowsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MediaMix => serializer.serialize_str("MEDIA_MIX"),
            Self::Category => serializer.serialize_str("CATEGORY"),
            Self::Funnel => serializer.serialize_str("FUNNEL"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InsightsAccountResponseRowsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MEDIA_MIX" => Ok(Self::MediaMix),
            "CATEGORY" => Ok(Self::Category),
            "FUNNEL" => Ok(Self::Funnel),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InsightsAccountResponseRowsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MediaMix => write!(f, "MEDIA_MIX"),
            Self::Category => write!(f, "CATEGORY"),
            Self::Funnel => write!(f, "FUNNEL"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

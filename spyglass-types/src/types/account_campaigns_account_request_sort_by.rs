pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignsAccountRequestSortBy {
    MostRecent,
    HighestVelocity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignsAccountRequestSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MostRecent => serializer.serialize_str("MOST_RECENT"),
            Self::HighestVelocity => serializer.serialize_str("HIGHEST_VELOCITY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignsAccountRequestSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MOST_RECENT" => Ok(Self::MostRecent),
            "HIGHEST_VELOCITY" => Ok(Self::HighestVelocity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignsAccountRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MostRecent => write!(f, "MOST_RECENT"),
            Self::HighestVelocity => write!(f, "HIGHEST_VELOCITY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

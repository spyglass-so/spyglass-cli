pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchBrandsRequestSortBy {
    Name,
    MostFollowers,
    LeastFollowers,
    MostViews,
    HighestEngagement,
    MostActiveAds,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchBrandsRequestSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Name => serializer.serialize_str("NAME"),
            Self::MostFollowers => serializer.serialize_str("MOST_FOLLOWERS"),
            Self::LeastFollowers => serializer.serialize_str("LEAST_FOLLOWERS"),
            Self::MostViews => serializer.serialize_str("MOST_VIEWS"),
            Self::HighestEngagement => serializer.serialize_str("HIGHEST_ENGAGEMENT"),
            Self::MostActiveAds => serializer.serialize_str("MOST_ACTIVE_ADS"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchBrandsRequestSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NAME" => Ok(Self::Name),
            "MOST_FOLLOWERS" => Ok(Self::MostFollowers),
            "LEAST_FOLLOWERS" => Ok(Self::LeastFollowers),
            "MOST_VIEWS" => Ok(Self::MostViews),
            "HIGHEST_ENGAGEMENT" => Ok(Self::HighestEngagement),
            "MOST_ACTIVE_ADS" => Ok(Self::MostActiveAds),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchBrandsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name => write!(f, "NAME"),
            Self::MostFollowers => write!(f, "MOST_FOLLOWERS"),
            Self::LeastFollowers => write!(f, "LEAST_FOLLOWERS"),
            Self::MostViews => write!(f, "MOST_VIEWS"),
            Self::HighestEngagement => write!(f, "HIGHEST_ENGAGEMENT"),
            Self::MostActiveAds => write!(f, "MOST_ACTIVE_ADS"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

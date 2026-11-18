pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaRequestSortBy {
    MostRecent,
    Oldest,
    LongestRun,
    MostImpressions,
    MostLikes,
    MostComments,
    MostViews,
    HighestEngagement,
    HighestOutlier,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaRequestSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MostRecent => serializer.serialize_str("MOST_RECENT"),
            Self::Oldest => serializer.serialize_str("OLDEST"),
            Self::LongestRun => serializer.serialize_str("LONGEST_RUN"),
            Self::MostImpressions => serializer.serialize_str("MOST_IMPRESSIONS"),
            Self::MostLikes => serializer.serialize_str("MOST_LIKES"),
            Self::MostComments => serializer.serialize_str("MOST_COMMENTS"),
            Self::MostViews => serializer.serialize_str("MOST_VIEWS"),
            Self::HighestEngagement => serializer.serialize_str("HIGHEST_ENGAGEMENT"),
            Self::HighestOutlier => serializer.serialize_str("HIGHEST_OUTLIER"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaRequestSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MOST_RECENT" => Ok(Self::MostRecent),
            "OLDEST" => Ok(Self::Oldest),
            "LONGEST_RUN" => Ok(Self::LongestRun),
            "MOST_IMPRESSIONS" => Ok(Self::MostImpressions),
            "MOST_LIKES" => Ok(Self::MostLikes),
            "MOST_COMMENTS" => Ok(Self::MostComments),
            "MOST_VIEWS" => Ok(Self::MostViews),
            "HIGHEST_ENGAGEMENT" => Ok(Self::HighestEngagement),
            "HIGHEST_OUTLIER" => Ok(Self::HighestOutlier),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MostRecent => write!(f, "MOST_RECENT"),
            Self::Oldest => write!(f, "OLDEST"),
            Self::LongestRun => write!(f, "LONGEST_RUN"),
            Self::MostImpressions => write!(f, "MOST_IMPRESSIONS"),
            Self::MostLikes => write!(f, "MOST_LIKES"),
            Self::MostComments => write!(f, "MOST_COMMENTS"),
            Self::MostViews => write!(f, "MOST_VIEWS"),
            Self::HighestEngagement => write!(f, "HIGHEST_ENGAGEMENT"),
            Self::HighestOutlier => write!(f, "HIGHEST_OUTLIER"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

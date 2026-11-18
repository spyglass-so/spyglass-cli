pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaRequestDatePreset {
    Today,
    Yesterday,
    Last3Days,
    Last7Days,
    ThisWeek,
    LastWeek,
    Last14Days,
    Last30Days,
    ThisMonth,
    LastMonth,
    Last3Months,
    Last6Months,
    YearToDate,
    LastYear,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaRequestDatePreset {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Today => serializer.serialize_str("today"),
            Self::Yesterday => serializer.serialize_str("yesterday"),
            Self::Last3Days => serializer.serialize_str("last-3-days"),
            Self::Last7Days => serializer.serialize_str("last-7-days"),
            Self::ThisWeek => serializer.serialize_str("this-week"),
            Self::LastWeek => serializer.serialize_str("last-week"),
            Self::Last14Days => serializer.serialize_str("last-14-days"),
            Self::Last30Days => serializer.serialize_str("last-30-days"),
            Self::ThisMonth => serializer.serialize_str("this-month"),
            Self::LastMonth => serializer.serialize_str("last-month"),
            Self::Last3Months => serializer.serialize_str("last-3-months"),
            Self::Last6Months => serializer.serialize_str("last-6-months"),
            Self::YearToDate => serializer.serialize_str("year-to-date"),
            Self::LastYear => serializer.serialize_str("last-year"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaRequestDatePreset {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "today" => Ok(Self::Today),
            "yesterday" => Ok(Self::Yesterday),
            "last-3-days" => Ok(Self::Last3Days),
            "last-7-days" => Ok(Self::Last7Days),
            "this-week" => Ok(Self::ThisWeek),
            "last-week" => Ok(Self::LastWeek),
            "last-14-days" => Ok(Self::Last14Days),
            "last-30-days" => Ok(Self::Last30Days),
            "this-month" => Ok(Self::ThisMonth),
            "last-month" => Ok(Self::LastMonth),
            "last-3-months" => Ok(Self::Last3Months),
            "last-6-months" => Ok(Self::Last6Months),
            "year-to-date" => Ok(Self::YearToDate),
            "last-year" => Ok(Self::LastYear),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaRequestDatePreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Today => write!(f, "today"),
            Self::Yesterday => write!(f, "yesterday"),
            Self::Last3Days => write!(f, "last-3-days"),
            Self::Last7Days => write!(f, "last-7-days"),
            Self::ThisWeek => write!(f, "this-week"),
            Self::LastWeek => write!(f, "last-week"),
            Self::Last14Days => write!(f, "last-14-days"),
            Self::Last30Days => write!(f, "last-30-days"),
            Self::ThisMonth => write!(f, "this-month"),
            Self::LastMonth => write!(f, "last-month"),
            Self::Last3Months => write!(f, "last-3-months"),
            Self::Last6Months => write!(f, "last-6-months"),
            Self::YearToDate => write!(f, "year-to-date"),
            Self::LastYear => write!(f, "last-year"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaRequestDurationItem {
    Zero10,
    Ten30,
    Thirty60,
    Sixty,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaRequestDurationItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Zero10 => serializer.serialize_str("0-10"),
            Self::Ten30 => serializer.serialize_str("10-30"),
            Self::Thirty60 => serializer.serialize_str("30-60"),
            Self::Sixty => serializer.serialize_str("60+"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaRequestDurationItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0-10" => Ok(Self::Zero10),
            "10-30" => Ok(Self::Ten30),
            "30-60" => Ok(Self::Thirty60),
            "60+" => Ok(Self::Sixty),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaRequestDurationItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero10 => write!(f, "0-10"),
            Self::Ten30 => write!(f, "10-30"),
            Self::Thirty60 => write!(f, "30-60"),
            Self::Sixty => write!(f, "60+"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

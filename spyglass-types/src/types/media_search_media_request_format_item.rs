pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaRequestFormatItem {
    Dpa,
    Video,
    Carousel,
    Image,
    Dco,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaRequestFormatItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dpa => serializer.serialize_str("DPA"),
            Self::Video => serializer.serialize_str("VIDEO"),
            Self::Carousel => serializer.serialize_str("CAROUSEL"),
            Self::Image => serializer.serialize_str("IMAGE"),
            Self::Dco => serializer.serialize_str("DCO"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaRequestFormatItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DPA" => Ok(Self::Dpa),
            "VIDEO" => Ok(Self::Video),
            "CAROUSEL" => Ok(Self::Carousel),
            "IMAGE" => Ok(Self::Image),
            "DCO" => Ok(Self::Dco),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaRequestFormatItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dpa => write!(f, "DPA"),
            Self::Video => write!(f, "VIDEO"),
            Self::Carousel => write!(f, "CAROUSEL"),
            Self::Image => write!(f, "IMAGE"),
            Self::Dco => write!(f, "DCO"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

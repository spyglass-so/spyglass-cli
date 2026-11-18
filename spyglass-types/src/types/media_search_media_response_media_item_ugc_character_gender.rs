pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaResponseMediaItemUgcCharacterGender {
    Male,
    Female,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaResponseMediaItemUgcCharacterGender {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Male => serializer.serialize_str("male"),
            Self::Female => serializer.serialize_str("female"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaResponseMediaItemUgcCharacterGender {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaResponseMediaItemUgcCharacterGender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Male => write!(f, "male"),
            Self::Female => write!(f, "female"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

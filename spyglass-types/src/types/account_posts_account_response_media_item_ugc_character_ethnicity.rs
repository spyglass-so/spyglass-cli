pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostsAccountResponseMediaItemUgcCharacterEthnicity {
    White,
    Black,
    Asian,
    Hispanic,
    MiddleEastern,
    Indigenous,
    PacificIslander,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostsAccountResponseMediaItemUgcCharacterEthnicity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::White => serializer.serialize_str("White"),
            Self::Black => serializer.serialize_str("Black"),
            Self::Asian => serializer.serialize_str("Asian"),
            Self::Hispanic => serializer.serialize_str("Hispanic"),
            Self::MiddleEastern => serializer.serialize_str("Middle Eastern"),
            Self::Indigenous => serializer.serialize_str("Indigenous"),
            Self::PacificIslander => serializer.serialize_str("Pacific Islander"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostsAccountResponseMediaItemUgcCharacterEthnicity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "White" => Ok(Self::White),
            "Black" => Ok(Self::Black),
            "Asian" => Ok(Self::Asian),
            "Hispanic" => Ok(Self::Hispanic),
            "Middle Eastern" => Ok(Self::MiddleEastern),
            "Indigenous" => Ok(Self::Indigenous),
            "Pacific Islander" => Ok(Self::PacificIslander),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostsAccountResponseMediaItemUgcCharacterEthnicity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => write!(f, "White"),
            Self::Black => write!(f, "Black"),
            Self::Asian => write!(f, "Asian"),
            Self::Hispanic => write!(f, "Hispanic"),
            Self::MiddleEastern => write!(f, "Middle Eastern"),
            Self::Indigenous => write!(f, "Indigenous"),
            Self::PacificIslander => write!(f, "Pacific Islander"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

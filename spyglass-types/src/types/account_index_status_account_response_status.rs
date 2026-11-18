pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexStatusAccountResponseStatus {
    Indexing,
    Ready,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IndexStatusAccountResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Indexing => serializer.serialize_str("indexing"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IndexStatusAccountResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "indexing" => Ok(Self::Indexing),
            "ready" => Ok(Self::Ready),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IndexStatusAccountResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Indexing => write!(f, "indexing"),
            Self::Ready => write!(f, "ready"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

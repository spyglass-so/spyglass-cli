pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexAccountResponseStatus {
    AlreadyExists,
    Indexing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IndexAccountResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AlreadyExists => serializer.serialize_str("already_exists"),
            Self::Indexing => serializer.serialize_str("indexing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IndexAccountResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "already_exists" => Ok(Self::AlreadyExists),
            "indexing" => Ok(Self::Indexing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IndexAccountResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists => write!(f, "already_exists"),
            Self::Indexing => write!(f, "indexing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

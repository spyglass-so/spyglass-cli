pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaRequestFunnelStageItem {
    Unaware,
    ProblemAware,
    SolutionAware,
    ProductAware,
    MostAware,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaRequestFunnelStageItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unaware => serializer.serialize_str("UNAWARE"),
            Self::ProblemAware => serializer.serialize_str("PROBLEM_AWARE"),
            Self::SolutionAware => serializer.serialize_str("SOLUTION_AWARE"),
            Self::ProductAware => serializer.serialize_str("PRODUCT_AWARE"),
            Self::MostAware => serializer.serialize_str("MOST_AWARE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaRequestFunnelStageItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "UNAWARE" => Ok(Self::Unaware),
            "PROBLEM_AWARE" => Ok(Self::ProblemAware),
            "SOLUTION_AWARE" => Ok(Self::SolutionAware),
            "PRODUCT_AWARE" => Ok(Self::ProductAware),
            "MOST_AWARE" => Ok(Self::MostAware),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaRequestFunnelStageItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unaware => write!(f, "UNAWARE"),
            Self::ProblemAware => write!(f, "PROBLEM_AWARE"),
            Self::SolutionAware => write!(f, "SOLUTION_AWARE"),
            Self::ProductAware => write!(f, "PRODUCT_AWARE"),
            Self::MostAware => write!(f, "MOST_AWARE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

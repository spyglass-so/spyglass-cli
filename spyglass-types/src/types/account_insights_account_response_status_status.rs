pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InsightsAccountResponseStatusStatus {
    #[serde(rename = "analyzing")]
    Analyzing,
}
impl fmt::Display for InsightsAccountResponseStatusStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Analyzing => "analyzing",
        };
        write!(f, "{}", s)
    }
}

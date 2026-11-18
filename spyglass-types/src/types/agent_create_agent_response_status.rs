pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateAgentResponseStatus {
    #[serde(rename = "queued")]
    Queued,
}
impl fmt::Display for CreateAgentResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Queued => "queued",
        };
        write!(f, "{}", s)
    }
}

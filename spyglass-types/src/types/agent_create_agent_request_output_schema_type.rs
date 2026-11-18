pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateAgentRequestOutputSchemaType {
    #[serde(rename = "object")]
    Object,
}
impl fmt::Display for CreateAgentRequestOutputSchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Object => "object",
        };
        write!(f, "{}", s)
    }
}

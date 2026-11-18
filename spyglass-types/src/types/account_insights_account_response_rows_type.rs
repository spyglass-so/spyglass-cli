pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InsightsAccountResponseRowsType {
    #[serde(rename = "MEDIA_MIX")]
    MediaMix,
}
impl fmt::Display for InsightsAccountResponseRowsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MediaMix => "MEDIA_MIX",
        };
        write!(f, "{}", s)
    }
}

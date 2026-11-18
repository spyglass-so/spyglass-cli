pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SearchCreatorsResponseCreatorsItemType {
    #[serde(rename = "CREATOR")]
    Creator,
}
impl fmt::Display for SearchCreatorsResponseCreatorsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Creator => "CREATOR",
        };
        write!(f, "{}", s)
    }
}

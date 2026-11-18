pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SearchBrandsResponseBrandsItemType {
    #[serde(rename = "BRAND")]
    Brand,
}
impl fmt::Display for SearchBrandsResponseBrandsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Brand => "BRAND",
        };
        write!(f, "{}", s)
    }
}

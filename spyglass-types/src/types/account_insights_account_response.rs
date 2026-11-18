pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InsightsAccountResponse {
        InsightsAccountResponseRows(InsightsAccountResponseRows),

        InsightsAccountResponseStatus(InsightsAccountResponseStatus),
}

impl InsightsAccountResponse {
    pub fn is_insights_account_response_rows(&self) -> bool {
        matches!(self, Self::InsightsAccountResponseRows(_))
    }

    pub fn is_insights_account_response_status(&self) -> bool {
        matches!(self, Self::InsightsAccountResponseStatus(_))
    }


    pub fn as_insights_account_response_rows(&self) -> Option<&InsightsAccountResponseRows> {
        match self {
                    Self::InsightsAccountResponseRows(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_insights_account_response_rows(self) -> Option<InsightsAccountResponseRows> {
        match self {
                    Self::InsightsAccountResponseRows(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_insights_account_response_status(&self) -> Option<&InsightsAccountResponseStatus> {
        match self {
                    Self::InsightsAccountResponseStatus(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_insights_account_response_status(self) -> Option<InsightsAccountResponseStatus> {
        match self {
                    Self::InsightsAccountResponseStatus(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for InsightsAccountResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsightsAccountResponseRows(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::InsightsAccountResponseStatus(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}

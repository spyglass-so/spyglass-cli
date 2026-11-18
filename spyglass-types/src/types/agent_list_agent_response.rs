pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAgentResponse {
    #[serde(default)]
    pub data: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub count: i64,
}

impl ListAgentResponse {
    pub fn builder() -> ListAgentResponseBuilder {
        <ListAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentResponseBuilder {
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
    count: Option<i64>,
}

impl ListAgentResponseBuilder {
    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAgentResponseBuilder::data)
    /// - [`count`](ListAgentResponseBuilder::count)
    pub fn build(self) -> Result<ListAgentResponse, BuildError> {
        Ok(ListAgentResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}

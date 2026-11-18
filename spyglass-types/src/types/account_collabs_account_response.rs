pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabsAccountResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub collabs: Vec<CollabsAccountResponseCollabsItem>,
}

impl CollabsAccountResponse {
    pub fn builder() -> CollabsAccountResponseBuilder {
        <CollabsAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabsAccountResponseBuilder {
    count: Option<i64>,
    collabs: Option<Vec<CollabsAccountResponseCollabsItem>>,
}

impl CollabsAccountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn collabs(mut self, value: Vec<CollabsAccountResponseCollabsItem>) -> Self {
        self.collabs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CollabsAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](CollabsAccountResponseBuilder::count)
    /// - [`collabs`](CollabsAccountResponseBuilder::collabs)
    pub fn build(self) -> Result<CollabsAccountResponse, BuildError> {
        Ok(CollabsAccountResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            collabs: self.collabs.ok_or_else(|| BuildError::missing_field("collabs"))?,
        })
    }
}

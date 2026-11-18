pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IndexStatusAccountResponse {
    pub status: IndexStatusAccountResponseStatus,
}

impl IndexStatusAccountResponse {
    pub fn builder() -> IndexStatusAccountResponseBuilder {
        <IndexStatusAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IndexStatusAccountResponseBuilder {
    status: Option<IndexStatusAccountResponseStatus>,
}

impl IndexStatusAccountResponseBuilder {
    pub fn status(mut self, value: IndexStatusAccountResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IndexStatusAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](IndexStatusAccountResponseBuilder::status)
    pub fn build(self) -> Result<IndexStatusAccountResponse, BuildError> {
        Ok(IndexStatusAccountResponse {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

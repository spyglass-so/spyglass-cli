pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IndexAccountResponse {
    #[serde(rename = "brandId")]
    #[serde(default)]
    pub brand_id: String,
    pub status: IndexAccountResponseStatus,
}

impl IndexAccountResponse {
    pub fn builder() -> IndexAccountResponseBuilder {
        <IndexAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IndexAccountResponseBuilder {
    brand_id: Option<String>,
    status: Option<IndexAccountResponseStatus>,
}

impl IndexAccountResponseBuilder {
    pub fn brand_id(mut self, value: impl Into<String>) -> Self {
        self.brand_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: IndexAccountResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IndexAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brand_id`](IndexAccountResponseBuilder::brand_id)
    /// - [`status`](IndexAccountResponseBuilder::status)
    pub fn build(self) -> Result<IndexAccountResponse, BuildError> {
        Ok(IndexAccountResponse {
            brand_id: self.brand_id.ok_or_else(|| BuildError::missing_field("brand_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

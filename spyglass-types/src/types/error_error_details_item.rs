pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorErrorDetailsItem {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub message: String,
}

impl ErrorErrorDetailsItem {
    pub fn builder() -> ErrorErrorDetailsItemBuilder {
        <ErrorErrorDetailsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorErrorDetailsItemBuilder {
    path: Option<String>,
    message: Option<String>,
}

impl ErrorErrorDetailsItemBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorErrorDetailsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](ErrorErrorDetailsItemBuilder::path)
    /// - [`message`](ErrorErrorDetailsItemBuilder::message)
    pub fn build(self) -> Result<ErrorErrorDetailsItem, BuildError> {
        Ok(ErrorErrorDetailsItem {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

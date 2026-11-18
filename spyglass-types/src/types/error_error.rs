pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ErrorError {
    pub code: ErrorErrorCode,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ErrorErrorDetailsItem>>,
}

impl ErrorError {
    pub fn builder() -> ErrorErrorBuilder {
        <ErrorErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorErrorBuilder {
    code: Option<ErrorErrorCode>,
    message: Option<String>,
    details: Option<Vec<ErrorErrorDetailsItem>>,
}

impl ErrorErrorBuilder {
    pub fn code(mut self, value: ErrorErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn details(mut self, value: Vec<ErrorErrorDetailsItem>) -> Self {
        self.details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ErrorError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ErrorErrorBuilder::code)
    /// - [`message`](ErrorErrorBuilder::message)
    pub fn build(self) -> Result<ErrorError, BuildError> {
        Ok(ErrorError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            details: self.details,
        })
    }
}

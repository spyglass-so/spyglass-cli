pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorErrorCode {
    InvalidRequest,
    InvalidApiKey,
    PayloadTooLarge,
    OutOfCredits,
    MembershipEnded,
    SubscriptionInactive,
    BrandNotFound,
    NotFound,
    MethodNotAllowed,
    RateLimited,
    NotImplemented,
    InternalError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ErrorErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InvalidRequest => serializer.serialize_str("invalid_request"),
            Self::InvalidApiKey => serializer.serialize_str("invalid_api_key"),
            Self::PayloadTooLarge => serializer.serialize_str("payload_too_large"),
            Self::OutOfCredits => serializer.serialize_str("out_of_credits"),
            Self::MembershipEnded => serializer.serialize_str("membership_ended"),
            Self::SubscriptionInactive => serializer.serialize_str("subscription_inactive"),
            Self::BrandNotFound => serializer.serialize_str("brand_not_found"),
            Self::NotFound => serializer.serialize_str("not_found"),
            Self::MethodNotAllowed => serializer.serialize_str("method_not_allowed"),
            Self::RateLimited => serializer.serialize_str("rate_limited"),
            Self::NotImplemented => serializer.serialize_str("not_implemented"),
            Self::InternalError => serializer.serialize_str("internal_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ErrorErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invalid_request" => Ok(Self::InvalidRequest),
            "invalid_api_key" => Ok(Self::InvalidApiKey),
            "payload_too_large" => Ok(Self::PayloadTooLarge),
            "out_of_credits" => Ok(Self::OutOfCredits),
            "membership_ended" => Ok(Self::MembershipEnded),
            "subscription_inactive" => Ok(Self::SubscriptionInactive),
            "brand_not_found" => Ok(Self::BrandNotFound),
            "not_found" => Ok(Self::NotFound),
            "method_not_allowed" => Ok(Self::MethodNotAllowed),
            "rate_limited" => Ok(Self::RateLimited),
            "not_implemented" => Ok(Self::NotImplemented),
            "internal_error" => Ok(Self::InternalError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ErrorErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest => write!(f, "invalid_request"),
            Self::InvalidApiKey => write!(f, "invalid_api_key"),
            Self::PayloadTooLarge => write!(f, "payload_too_large"),
            Self::OutOfCredits => write!(f, "out_of_credits"),
            Self::MembershipEnded => write!(f, "membership_ended"),
            Self::SubscriptionInactive => write!(f, "subscription_inactive"),
            Self::BrandNotFound => write!(f, "brand_not_found"),
            Self::NotFound => write!(f, "not_found"),
            Self::MethodNotAllowed => write!(f, "method_not_allowed"),
            Self::RateLimited => write!(f, "rate_limited"),
            Self::NotImplemented => write!(f, "not_implemented"),
            Self::InternalError => write!(f, "internal_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

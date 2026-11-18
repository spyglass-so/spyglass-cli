pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditUsageTeamResponse {
    /// Credits left in this billing period, including granted credits.
    #[serde(rename = "remainingCredits")]
    #[serde(default)]
    pub remaining_credits: i64,
    /// Credits included in the team's plan for this billing period.
    #[serde(rename = "planCredits")]
    #[serde(default)]
    pub plan_credits: i64,
    /// Start of the current billing period, UTC. Null when no billing period exists.
    #[serde(rename = "billingPeriodStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<String>,
    /// End of the current billing period, UTC. Null when no billing period exists.
    #[serde(rename = "billingPeriodEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<String>,
}

impl CreditUsageTeamResponse {
    pub fn builder() -> CreditUsageTeamResponseBuilder {
        <CreditUsageTeamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditUsageTeamResponseBuilder {
    remaining_credits: Option<i64>,
    plan_credits: Option<i64>,
    billing_period_start: Option<String>,
    billing_period_end: Option<String>,
}

impl CreditUsageTeamResponseBuilder {
    pub fn remaining_credits(mut self, value: i64) -> Self {
        self.remaining_credits = Some(value);
        self
    }

    pub fn plan_credits(mut self, value: i64) -> Self {
        self.plan_credits = Some(value);
        self
    }

    pub fn billing_period_start(mut self, value: impl Into<String>) -> Self {
        self.billing_period_start = Some(value.into());
        self
    }

    pub fn billing_period_end(mut self, value: impl Into<String>) -> Self {
        self.billing_period_end = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditUsageTeamResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`remaining_credits`](CreditUsageTeamResponseBuilder::remaining_credits)
    /// - [`plan_credits`](CreditUsageTeamResponseBuilder::plan_credits)
    pub fn build(self) -> Result<CreditUsageTeamResponse, BuildError> {
        Ok(CreditUsageTeamResponse {
            remaining_credits: self.remaining_credits.ok_or_else(|| BuildError::missing_field("remaining_credits"))?,
            plan_credits: self.plan_credits.ok_or_else(|| BuildError::missing_field("plan_credits"))?,
            billing_period_start: self.billing_period_start,
            billing_period_end: self.billing_period_end,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignsAccountResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub campaigns: Vec<CampaignsAccountResponseCampaignsItem>,
}

impl CampaignsAccountResponse {
    pub fn builder() -> CampaignsAccountResponseBuilder {
        <CampaignsAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsAccountResponseBuilder {
    count: Option<i64>,
    campaigns: Option<Vec<CampaignsAccountResponseCampaignsItem>>,
}

impl CampaignsAccountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn campaigns(mut self, value: Vec<CampaignsAccountResponseCampaignsItem>) -> Self {
        self.campaigns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](CampaignsAccountResponseBuilder::count)
    /// - [`campaigns`](CampaignsAccountResponseBuilder::campaigns)
    pub fn build(self) -> Result<CampaignsAccountResponse, BuildError> {
        Ok(CampaignsAccountResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            campaigns: self.campaigns.ok_or_else(|| BuildError::missing_field("campaigns"))?,
        })
    }
}

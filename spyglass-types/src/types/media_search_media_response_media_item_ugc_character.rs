pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchMediaResponseMediaItemUgcCharacter {
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<SearchMediaResponseMediaItemUgcCharacterAgeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<SearchMediaResponseMediaItemUgcCharacterGender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<SearchMediaResponseMediaItemUgcCharacterEthnicity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SearchMediaResponseMediaItemUgcCharacter {
    pub fn builder() -> SearchMediaResponseMediaItemUgcCharacterBuilder {
        <SearchMediaResponseMediaItemUgcCharacterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMediaResponseMediaItemUgcCharacterBuilder {
    age_range: Option<SearchMediaResponseMediaItemUgcCharacterAgeRange>,
    gender: Option<SearchMediaResponseMediaItemUgcCharacterGender>,
    ethnicity: Option<SearchMediaResponseMediaItemUgcCharacterEthnicity>,
    description: Option<String>,
}

impl SearchMediaResponseMediaItemUgcCharacterBuilder {
    pub fn age_range(mut self, value: SearchMediaResponseMediaItemUgcCharacterAgeRange) -> Self {
        self.age_range = Some(value);
        self
    }

    pub fn gender(mut self, value: SearchMediaResponseMediaItemUgcCharacterGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn ethnicity(mut self, value: SearchMediaResponseMediaItemUgcCharacterEthnicity) -> Self {
        self.ethnicity = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchMediaResponseMediaItemUgcCharacter`].
    pub fn build(self) -> Result<SearchMediaResponseMediaItemUgcCharacter, BuildError> {
        Ok(SearchMediaResponseMediaItemUgcCharacter {
            age_range: self.age_range,
            gender: self.gender,
            ethnicity: self.ethnicity,
            description: self.description,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostsAccountResponseMediaItemUgcCharacter {
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<PostsAccountResponseMediaItemUgcCharacterAgeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<PostsAccountResponseMediaItemUgcCharacterGender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<PostsAccountResponseMediaItemUgcCharacterEthnicity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostsAccountResponseMediaItemUgcCharacter {
    pub fn builder() -> PostsAccountResponseMediaItemUgcCharacterBuilder {
        <PostsAccountResponseMediaItemUgcCharacterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostsAccountResponseMediaItemUgcCharacterBuilder {
    age_range: Option<PostsAccountResponseMediaItemUgcCharacterAgeRange>,
    gender: Option<PostsAccountResponseMediaItemUgcCharacterGender>,
    ethnicity: Option<PostsAccountResponseMediaItemUgcCharacterEthnicity>,
    description: Option<String>,
}

impl PostsAccountResponseMediaItemUgcCharacterBuilder {
    pub fn age_range(mut self, value: PostsAccountResponseMediaItemUgcCharacterAgeRange) -> Self {
        self.age_range = Some(value);
        self
    }

    pub fn gender(mut self, value: PostsAccountResponseMediaItemUgcCharacterGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn ethnicity(mut self, value: PostsAccountResponseMediaItemUgcCharacterEthnicity) -> Self {
        self.ethnicity = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostsAccountResponseMediaItemUgcCharacter`].
    pub fn build(self) -> Result<PostsAccountResponseMediaItemUgcCharacter, BuildError> {
        Ok(PostsAccountResponseMediaItemUgcCharacter {
            age_range: self.age_range,
            gender: self.gender,
            ethnicity: self.ethnicity,
            description: self.description,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostsAccountResponseMediaItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "platformId")]
    #[serde(default)]
    pub platform_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<PostsAccountResponseMediaItemBrand>,
    /// Landing page headline. For organic posts, the post title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Call to action, such as Shop now or Learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<String>,
    /// Landing page link for a Meta ad. Null for Instagram and TikTok posts.
    #[serde(rename = "landingPageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page_url: Option<String>,
    /// Where a Meta ad was shown, such as FACEBOOK, INSTAGRAM, or AUDIENCE_NETWORK. Present only on Meta ads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Vec<PostsAccountResponseMediaItemPlacementItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostsAccountResponseMediaItemType>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<PostsAccountResponseMediaItemMediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    pub urls: Vec<String>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub height: Option<f64>,
    #[serde(rename = "durationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_in_seconds: Option<f64>,
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "coreAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_angle: Option<String>,
    #[serde(rename = "targetPersona")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_persona: Option<String>,
    #[serde(rename = "painPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pain_points: Option<Vec<String>>,
    #[serde(rename = "sellingPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_points: Option<Vec<String>>,
    #[serde(rename = "ugcCharacter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugc_character: Option<PostsAccountResponseMediaItemUgcCharacter>,
    #[serde(rename = "startAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "endAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    #[serde(rename = "daysRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub days_run: Option<f64>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(rename = "carouselCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub carousel_count: Option<f64>,
    #[serde(rename = "viewCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub view_count: Option<f64>,
    #[serde(rename = "likeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub like_count: Option<f64>,
    #[serde(rename = "commentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub comment_count: Option<f64>,
    #[serde(rename = "impressionPercentile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub impression_percentile: Option<f64>,
    #[serde(rename = "impressionsText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

impl PostsAccountResponseMediaItem {
    pub fn builder() -> PostsAccountResponseMediaItemBuilder {
        <PostsAccountResponseMediaItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostsAccountResponseMediaItemBuilder {
    id: Option<String>,
    platform_id: Option<String>,
    brand: Option<PostsAccountResponseMediaItemBrand>,
    title: Option<String>,
    cta: Option<String>,
    landing_page_url: Option<String>,
    placement: Option<Vec<PostsAccountResponseMediaItemPlacementItem>>,
    r#type: Option<PostsAccountResponseMediaItemType>,
    media_type: Option<PostsAccountResponseMediaItemMediaType>,
    url: Option<String>,
    urls: Option<Vec<String>>,
    thumbnail_url: Option<String>,
    width: Option<f64>,
    height: Option<f64>,
    duration_in_seconds: Option<f64>,
    platforms: Option<Vec<String>>,
    hook: Option<String>,
    summary: Option<String>,
    core_angle: Option<String>,
    target_persona: Option<String>,
    pain_points: Option<Vec<String>>,
    selling_points: Option<Vec<String>>,
    ugc_character: Option<PostsAccountResponseMediaItemUgcCharacter>,
    start_at: Option<String>,
    end_at: Option<String>,
    days_run: Option<f64>,
    categories: Option<Vec<String>>,
    carousel_count: Option<f64>,
    view_count: Option<f64>,
    like_count: Option<f64>,
    comment_count: Option<f64>,
    impression_percentile: Option<f64>,
    impressions_text: Option<String>,
    transcript: Option<String>,
    caption: Option<String>,
}

impl PostsAccountResponseMediaItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn platform_id(mut self, value: impl Into<String>) -> Self {
        self.platform_id = Some(value.into());
        self
    }

    pub fn brand(mut self, value: PostsAccountResponseMediaItemBrand) -> Self {
        self.brand = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn cta(mut self, value: impl Into<String>) -> Self {
        self.cta = Some(value.into());
        self
    }

    pub fn landing_page_url(mut self, value: impl Into<String>) -> Self {
        self.landing_page_url = Some(value.into());
        self
    }

    pub fn placement(mut self, value: Vec<PostsAccountResponseMediaItemPlacementItem>) -> Self {
        self.placement = Some(value);
        self
    }

    pub fn r#type(mut self, value: PostsAccountResponseMediaItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn media_type(mut self, value: PostsAccountResponseMediaItemMediaType) -> Self {
        self.media_type = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn urls(mut self, value: Vec<String>) -> Self {
        self.urls = Some(value);
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn duration_in_seconds(mut self, value: f64) -> Self {
        self.duration_in_seconds = Some(value);
        self
    }

    pub fn platforms(mut self, value: Vec<String>) -> Self {
        self.platforms = Some(value);
        self
    }

    pub fn hook(mut self, value: impl Into<String>) -> Self {
        self.hook = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn core_angle(mut self, value: impl Into<String>) -> Self {
        self.core_angle = Some(value.into());
        self
    }

    pub fn target_persona(mut self, value: impl Into<String>) -> Self {
        self.target_persona = Some(value.into());
        self
    }

    pub fn pain_points(mut self, value: Vec<String>) -> Self {
        self.pain_points = Some(value);
        self
    }

    pub fn selling_points(mut self, value: Vec<String>) -> Self {
        self.selling_points = Some(value);
        self
    }

    pub fn ugc_character(mut self, value: PostsAccountResponseMediaItemUgcCharacter) -> Self {
        self.ugc_character = Some(value);
        self
    }

    pub fn start_at(mut self, value: impl Into<String>) -> Self {
        self.start_at = Some(value.into());
        self
    }

    pub fn end_at(mut self, value: impl Into<String>) -> Self {
        self.end_at = Some(value.into());
        self
    }

    pub fn days_run(mut self, value: f64) -> Self {
        self.days_run = Some(value);
        self
    }

    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn carousel_count(mut self, value: f64) -> Self {
        self.carousel_count = Some(value);
        self
    }

    pub fn view_count(mut self, value: f64) -> Self {
        self.view_count = Some(value);
        self
    }

    pub fn like_count(mut self, value: f64) -> Self {
        self.like_count = Some(value);
        self
    }

    pub fn comment_count(mut self, value: f64) -> Self {
        self.comment_count = Some(value);
        self
    }

    pub fn impression_percentile(mut self, value: f64) -> Self {
        self.impression_percentile = Some(value);
        self
    }

    pub fn impressions_text(mut self, value: impl Into<String>) -> Self {
        self.impressions_text = Some(value.into());
        self
    }

    pub fn transcript(mut self, value: impl Into<String>) -> Self {
        self.transcript = Some(value.into());
        self
    }

    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostsAccountResponseMediaItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostsAccountResponseMediaItemBuilder::id)
    /// - [`platform_id`](PostsAccountResponseMediaItemBuilder::platform_id)
    /// - [`urls`](PostsAccountResponseMediaItemBuilder::urls)
    /// - [`platforms`](PostsAccountResponseMediaItemBuilder::platforms)
    /// - [`categories`](PostsAccountResponseMediaItemBuilder::categories)
    pub fn build(self) -> Result<PostsAccountResponseMediaItem, BuildError> {
        Ok(PostsAccountResponseMediaItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            platform_id: self.platform_id.ok_or_else(|| BuildError::missing_field("platform_id"))?,
            brand: self.brand,
            title: self.title,
            cta: self.cta,
            landing_page_url: self.landing_page_url,
            placement: self.placement,
            r#type: self.r#type,
            media_type: self.media_type,
            url: self.url,
            urls: self.urls.ok_or_else(|| BuildError::missing_field("urls"))?,
            thumbnail_url: self.thumbnail_url,
            width: self.width,
            height: self.height,
            duration_in_seconds: self.duration_in_seconds,
            platforms: self.platforms.ok_or_else(|| BuildError::missing_field("platforms"))?,
            hook: self.hook,
            summary: self.summary,
            core_angle: self.core_angle,
            target_persona: self.target_persona,
            pain_points: self.pain_points,
            selling_points: self.selling_points,
            ugc_character: self.ugc_character,
            start_at: self.start_at,
            end_at: self.end_at,
            days_run: self.days_run,
            categories: self.categories.ok_or_else(|| BuildError::missing_field("categories"))?,
            carousel_count: self.carousel_count,
            view_count: self.view_count,
            like_count: self.like_count,
            comment_count: self.comment_count,
            impression_percentile: self.impression_percentile,
            impressions_text: self.impressions_text,
            transcript: self.transcript,
            caption: self.caption,
        })
    }
}

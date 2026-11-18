pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMediaRequestCategoriesItem {
    Announcement,
    BeforeAndAfter,
    BrandStory,
    CaseStudy,
    Comparison,
    DiscountOrOffer,
    Explainer,
    FounderStory,
    Interface,
    Meme,
    NegativeReview,
    Press,
    ProductShowcase,
    Quiz,
    Quote,
    SocialProof,
    Testimonial,
    FakeInterface,
    Listicle,
    Interview,
    Unboxing,
    Grwm,
    DayInTheLife,
    Storytime,
    Skit,
    Reply,
    TextOverlay,
    Greenscreen,
    Pov,
    TryOnHaul,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchMediaRequestCategoriesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Announcement => serializer.serialize_str("Announcement"),
            Self::BeforeAndAfter => serializer.serialize_str("Before and After"),
            Self::BrandStory => serializer.serialize_str("Brand Story"),
            Self::CaseStudy => serializer.serialize_str("Case Study"),
            Self::Comparison => serializer.serialize_str("Comparison"),
            Self::DiscountOrOffer => serializer.serialize_str("Discount or Offer"),
            Self::Explainer => serializer.serialize_str("Explainer"),
            Self::FounderStory => serializer.serialize_str("Founder Story"),
            Self::Interface => serializer.serialize_str("Interface"),
            Self::Meme => serializer.serialize_str("Meme"),
            Self::NegativeReview => serializer.serialize_str("Negative Review"),
            Self::Press => serializer.serialize_str("Press"),
            Self::ProductShowcase => serializer.serialize_str("Product Showcase"),
            Self::Quiz => serializer.serialize_str("Quiz"),
            Self::Quote => serializer.serialize_str("Quote"),
            Self::SocialProof => serializer.serialize_str("Social Proof"),
            Self::Testimonial => serializer.serialize_str("Testimonial"),
            Self::FakeInterface => serializer.serialize_str("Fake Interface"),
            Self::Listicle => serializer.serialize_str("Listicle"),
            Self::Interview => serializer.serialize_str("Interview"),
            Self::Unboxing => serializer.serialize_str("Unboxing"),
            Self::Grwm => serializer.serialize_str("GRWM"),
            Self::DayInTheLife => serializer.serialize_str("Day in the life"),
            Self::Storytime => serializer.serialize_str("Storytime"),
            Self::Skit => serializer.serialize_str("Skit"),
            Self::Reply => serializer.serialize_str("Reply"),
            Self::TextOverlay => serializer.serialize_str("Text Overlay"),
            Self::Greenscreen => serializer.serialize_str("Greenscreen"),
            Self::Pov => serializer.serialize_str("POV"),
            Self::TryOnHaul => serializer.serialize_str("Try-on / Haul"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchMediaRequestCategoriesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Announcement" => Ok(Self::Announcement),
            "Before and After" => Ok(Self::BeforeAndAfter),
            "Brand Story" => Ok(Self::BrandStory),
            "Case Study" => Ok(Self::CaseStudy),
            "Comparison" => Ok(Self::Comparison),
            "Discount or Offer" => Ok(Self::DiscountOrOffer),
            "Explainer" => Ok(Self::Explainer),
            "Founder Story" => Ok(Self::FounderStory),
            "Interface" => Ok(Self::Interface),
            "Meme" => Ok(Self::Meme),
            "Negative Review" => Ok(Self::NegativeReview),
            "Press" => Ok(Self::Press),
            "Product Showcase" => Ok(Self::ProductShowcase),
            "Quiz" => Ok(Self::Quiz),
            "Quote" => Ok(Self::Quote),
            "Social Proof" => Ok(Self::SocialProof),
            "Testimonial" => Ok(Self::Testimonial),
            "Fake Interface" => Ok(Self::FakeInterface),
            "Listicle" => Ok(Self::Listicle),
            "Interview" => Ok(Self::Interview),
            "Unboxing" => Ok(Self::Unboxing),
            "GRWM" => Ok(Self::Grwm),
            "Day in the life" => Ok(Self::DayInTheLife),
            "Storytime" => Ok(Self::Storytime),
            "Skit" => Ok(Self::Skit),
            "Reply" => Ok(Self::Reply),
            "Text Overlay" => Ok(Self::TextOverlay),
            "Greenscreen" => Ok(Self::Greenscreen),
            "POV" => Ok(Self::Pov),
            "Try-on / Haul" => Ok(Self::TryOnHaul),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchMediaRequestCategoriesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Announcement => write!(f, "Announcement"),
            Self::BeforeAndAfter => write!(f, "Before and After"),
            Self::BrandStory => write!(f, "Brand Story"),
            Self::CaseStudy => write!(f, "Case Study"),
            Self::Comparison => write!(f, "Comparison"),
            Self::DiscountOrOffer => write!(f, "Discount or Offer"),
            Self::Explainer => write!(f, "Explainer"),
            Self::FounderStory => write!(f, "Founder Story"),
            Self::Interface => write!(f, "Interface"),
            Self::Meme => write!(f, "Meme"),
            Self::NegativeReview => write!(f, "Negative Review"),
            Self::Press => write!(f, "Press"),
            Self::ProductShowcase => write!(f, "Product Showcase"),
            Self::Quiz => write!(f, "Quiz"),
            Self::Quote => write!(f, "Quote"),
            Self::SocialProof => write!(f, "Social Proof"),
            Self::Testimonial => write!(f, "Testimonial"),
            Self::FakeInterface => write!(f, "Fake Interface"),
            Self::Listicle => write!(f, "Listicle"),
            Self::Interview => write!(f, "Interview"),
            Self::Unboxing => write!(f, "Unboxing"),
            Self::Grwm => write!(f, "GRWM"),
            Self::DayInTheLife => write!(f, "Day in the life"),
            Self::Storytime => write!(f, "Storytime"),
            Self::Skit => write!(f, "Skit"),
            Self::Reply => write!(f, "Reply"),
            Self::TextOverlay => write!(f, "Text Overlay"),
            Self::Greenscreen => write!(f, "Greenscreen"),
            Self::Pov => write!(f, "POV"),
            Self::TryOnHaul => write!(f, "Try-on / Haul"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

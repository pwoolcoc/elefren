//! Module representing cards of statuses.

/// A card of a status.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Entity)]
pub struct Card {
    /// Location of linked resource.
    url: String,
    /// Title of linked resource.
    title: String,
    /// Description of preview.
    description: String,
    #[cfg(feature = "mastodon_1_3_0")]
    /// The type of the preview card.
    r#type: CardType,
    #[cfg(feature = "mastodon_1_3_0")]
    /// The author of the original resource.
    author_name: Option<String>,
    #[cfg(feature = "mastodon_1_3_0")]
    /// A link to the author of the original resource.
    author_url: Option<String>,
    #[cfg(feature = "mastodon_1_3_0")]
    /// The provider of the original resource.
    provider_name: Option<String>,
    #[cfg(feature = "mastodon_1_3_0")]
    /// A link to the provider of the original resource.
    provider_url: Option<String>,
    #[cfg(feature = "mastodon_1_3_0")]
    /// HTML to be used for generating the preview card.
    html: Option<String>,
    #[cfg(feature = "mastodon_1_3_0")]
    /// Width of preview, in pixels.
    width: Option<u64>,
    #[cfg(feature = "mastodon_1_3_0")]
    /// Height of preview, in pixels.
    height: Option<u64>,
    /// Preview thumbnail.
    image: Option<String>,
    #[cfg(feature = "mastodon_2_1_0")]
    /// Used for photo embeds, instead of custom html.
    embed_url: Option<String>,
    #[cfg(feature = "mastodon_3_2_0")]
    /// A hash computed by the BlurHash algorithm, for generating colorful preview thumbnails when media has not been downloaded yet.
    blurhash: Option<String>,

    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}
impl Card {
    /// Location of linked resource.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// Title of linked resource.
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Description of preview.
    pub fn description(&self) -> &str {
        &self.description
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// The type of the preview card.
    pub fn r#type(&self) -> &CardType {
        &self.r#type
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// The author of the original resource.
    pub fn author_name(&self) -> Option<&String> {
        self.author_name.as_ref()
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// A link to the author of the original resource.
    pub fn author_url(&self) -> Option<&String> {
        self.author_url.as_ref()
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// The provider of the original resource.
    pub fn provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// A link to the provider of the original resource.
    pub fn provider_url(&self) -> Option<&String> {
        self.provider_url.as_ref()
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// HTML to be used for generating the preview card.
    pub fn html(&self) -> Option<&String> {
        self.html.as_ref()
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// Width of preview, in pixels.
    pub fn width(&self) -> Option<u64> {
        self.width
    }
    #[cfg(feature = "mastodon_1_3_0")]
    /// Height of preview, in pixels.
    pub fn height(&self) -> Option<u64> {
        self.height
    }
    /// Preview thumbnail.
    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }
    #[cfg(feature = "mastodon_2_1_0")]
    /// Used for photo embeds, instead of custom html.
    pub fn embed_url(&self) -> Option<&String> {
        self.embed_url.as_ref()
    }
    #[cfg(feature = "mastodon_3_2_0")]
    /// A hash computed by the BlurHash algorithm, for generating colorful preview thumbnails when media has not been downloaded yet.
    pub fn blurhash(&self) -> Option<&String> {
        self.blurhash.as_ref()
    }
}

/// The possible card types
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg(feature = "mastodon_1_3_0")]
pub enum CardType {
    /// Link
    Link,
    /// Phot
    Photo,
    /// Video
    Video,
    /// Rich
    Rich,
}

use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use derive_entity::Entity;

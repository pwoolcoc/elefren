//! Module containing everything related to media attachements.

/// A struct representing a media attachment.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Entity)]
pub struct Attachment {
    id: String,
    r#type: MediaType,
    url: String,
    preview_url: String,
    remote_url: Option<String>,
    text_url: Option<String>,
    meta: Option<Meta>,
    #[cfg(feature = "mastodon_2_0_0")]
    description: Option<String>,
    #[cfg(feature = "mastodon_2_8_1")]
    blurhash: String,

    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}
impl Attachment {
    /// ID of the attachment.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// The media type of an attachment.
    pub fn r#type(&self) -> &MediaType {
        &self.r#type
    }
    /// URL of the locally hosted version of the image.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// For remote images, the remote URL of the original image.
    pub fn remote_url(&self) -> Option<&String> {
        self.remote_url.as_ref()
    }
    /// URL of the preview image.
    pub fn preview_url(&self) -> &str {
        &self.preview_url
    }
    /// Shorter URL for the image, for insertion into text
    /// (only present on local images)
    pub fn text_url(&self) -> Option<&String> {
        self.text_url.as_ref()
    }
    /// Meta information about the attachment.
    pub fn meta(&self) -> Option<&Meta> {
        self.meta.as_ref()
    }
    #[cfg(feature = "mastodon_2_0_0")]
    /// Alternate text that describes what is in the media attachment, to be used for the visually impaired or when media attachments do not load.
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    #[cfg(feature = "mastodon_2_8_1")]
    /// A hash computed by the BlurHash algorithm, for generating colorful preview thumbnails when media has not been downloaded yet.
    pub fn blurhash(&self) -> &str {
        &self.blurhash
    }
}

/// Information about the attachment itself.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Entity)]
pub struct Meta {
    original: Option<ImageDetails>,
    small: Option<ImageDetails>,
    #[cfg(feature = "mastodon_2_3_0")]
    focus: Option<Focus>,
    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}
impl Meta {
    /// Original version.
    pub fn original(&self) -> Option<&ImageDetails> {
        self.original.as_ref()
    }
    /// Smaller version.
    pub fn small(&self) -> Option<&ImageDetails> {
        self.small.as_ref()
    }
    #[cfg(feature = "mastodon_2_3_0")]
    /// Coordinates for thumbnail cropping
    pub fn focus(&self) -> Option<&Focus> {
        self.focus.as_ref()
    }
}

#[cfg(feature = "mastodon_2_3_0")]
/// Focal point for an image
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Entity)]
pub struct Focus {
    x: f64,
    y: f64,
    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}
#[cfg(feature = "mastodon_2_3_0")]
impl Focus {
    /// X coordinate
    pub fn x(&self) -> f64 {
        self.x
    }
    /// Y coordinate
    pub fn y(&self) -> f64 {
        self.y
    }
}

/// Dimensions of an attachement.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Entity)]
pub struct ImageDetails {
    width: u64,
    height: u64,
    size: Option<String>,
    aspect: Option<f64>,

    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}
impl ImageDetails {
    /// width of attachment.
    pub fn width(&self) -> u64 {
        self.width
    }
    /// height of attachment.
    pub fn height(&self) -> u64 {
        self.height
    }
    /// A string of `widthxheight`.
    pub fn size(&self) -> Option<&String> {
        self.size.as_ref()
    }
    /// The aspect ratio of the attachment.
    pub fn aspect(&self) -> Option<&f64> {
        self.aspect.as_ref()
    }
}

/// The type of media attachment.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum MediaType {
    /// An image.
    #[serde(rename = "image")]
    Image,
    /// A video file.
    #[serde(rename = "video")]
    Video,
    /// A gifv format file.
    #[serde(rename = "gifv")]
    Gifv,
    #[cfg(feature = "mastodon_2_9_1")]
    #[serde(rename = "audio")]
    /// A audio file.
    Audio,
    /// Unknown format.
    #[serde(rename = "unknown")]
    Unknown,
}

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use derive_entity::Entity;

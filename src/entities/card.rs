//! Module representing cards of statuses.
use serde::Deserialize;

/// A card of a status.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Card {
    /// The url associated with the card.
    pub url: String,
    /// The title of the card.
    pub title: String,
    /// The card description.
    pub description: String,
    /// The image associated with the card, if any.
    pub image: Option<String>,
    /// OEmbed data
    author_name: Option<String>,
    /// OEmbed data
    author_url: Option<String>,
    /// OEmbed data
    provider_name: Option<String>,
    /// OEmbed data
    provider_url: Option<String>,
    /// OEmbed data
    html: Option<String>,
    /// OEmbed data
    width: Option<u64>,
    /// OEmbed data
    height: Option<u64>,
}

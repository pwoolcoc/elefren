//! Module containing everything related to an instance.
use super::account::Account;
use serde::Deserialize;

/// A struct containing info of an instance.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Instance {
    /// URI of the current instance
    pub uri: String,
    /// The instance's title.
    pub title: String,
    /// A description for the instance.
    pub description: String,
    /// An email address which can be used to contact the
    /// instance administrator.
    pub email: String,
    /// The Mastodon version used by instance.
    pub version: String,
    /// Urls to the streaming api.
    pub urls: Option<StreamingApi>,
    /// Stats about the instance.
    pub stats: Option<Stats>,
    /// Thumbnail of the server image.
    pub thumbnail: Option<String>,
    /// List of languages used on the server.
    pub languages: Option<Vec<String>>,
    /// Contact account for the server.
    pub contact_account: Option<Account>,
    /// The maximum number of characters allowed in a status
    pub max_toot_chars: Option<u32>,
}

/// Object containing url for streaming api.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct StreamingApi {
    /// Url for streaming API, typically a `wss://` url.
    pub streaming_api: String,
}

/// Statistics about the Mastodon instance.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct Stats {
    user_count: u64,
    status_count: u64,
    domain_count: u64,
}

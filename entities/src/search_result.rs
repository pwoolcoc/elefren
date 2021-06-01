//! A module containing info relating to a search result.
use serde::Deserialize;

use super::{
    prelude::{Account, Status},
    status::Tag,
};

/// A struct containing results of a search.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SearchResult {
    /// An array of matched Accounts.
    pub accounts: Vec<Account>,
    /// An array of matched Statuses.
    pub statuses: Vec<Status>,
    /// An array of matched hashtags, as strings.
    pub hashtags: Vec<String>,
}

/// A struct containing results of a search, with `Tag` objects in the
/// `hashtags` field
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SearchResultV2 {
    /// An array of matched Accounts.
    pub accounts: Vec<Account>,
    /// An array of matched Statuses.
    pub statuses: Vec<Status>,
    /// An array of matched hashtags, as `Tag` objects.
    pub hashtags: Vec<Tag>,
}

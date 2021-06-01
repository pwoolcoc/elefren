/// The visibility of a status.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    /// A Direct message to a user
    Direct,
    /// Only available to followers
    Private,
    /// Not shown in public timelines
    Unlisted,
    /// Posted to public timelines
    Public,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

use serde::{Deserialize, Serialize};
use std::default::Default;

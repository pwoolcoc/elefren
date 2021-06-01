use serde::{Deserialize, Serialize};

/// Represents a single Filter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    id: String,
    phrase: String,
    context: Vec<FilterContext>,
    expires_at: Option<String>, // TODO: timestamp
    irreversible: bool,
    whole_word: bool,
}

/// Represents the various types of Filter contexts
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FilterContext {
    /// Represents the "home" context
    #[serde(rename = "home")]
    Home,
    /// Represents the "notifications" context
    #[serde(rename = "notifications")]
    Notifications,
    /// Represents the "public" context
    #[serde(rename = "public")]
    Public,
    /// Represents the "thread" context
    #[serde(rename = "thread")]
    Thread,
}

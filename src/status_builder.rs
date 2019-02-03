use isolang::Language;
use std::fmt;

/// A builder pattern struct for constructing a status.
///
/// # Example
///
/// ```
/// # extern crate elefren;
/// # use elefren::{Language, StatusBuilder};
///
/// let status = StatusBuilder {
///     status: "a status".to_string(),
///     sensitive: Some(true),
///     spoiler_text: Some("a CW".to_string()),
///     language: Some(Language::Eng),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct StatusBuilder {
    /// The text of the status.
    pub status: String,
    /// Ids of accounts being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<u64>,
    /// Ids of media attachments being attached to the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
    /// Whether current status is sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// Text to precede the normal status text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiler_text: Option<String>,
    /// Visibility of the status, defaults to `Public`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /// Language code of the status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
}

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

impl StatusBuilder {
    /// Create a new status with text.
    /// ```
    /// use elefren::prelude::*;
    ///
    /// let status = StatusBuilder::new("Hello World!");
    /// ```
    pub fn new<D: fmt::Display>(status: D) -> Self {
        StatusBuilder {
            status: status.to_string(),
            ..Self::default()
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use isolang::Language;
    use serde_json;

    #[test]
    fn test_new() {
        let s = StatusBuilder::new("a status");
        let expected = StatusBuilder {
            status: "a status".to_string(),
            in_reply_to_id: None,
            media_ids: None,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
            language: None,
        };
        assert_eq!(s, expected);
    }

    #[test]
    fn test_default_visibility() {
        let v: Visibility = Default::default();
        assert_eq!(v, Visibility::Public);
    }

    #[test]
    fn test_serialize_visibility() {
        assert_eq!(
            serde_json::to_string(&Visibility::Direct).expect("couldn't serialize visibility"),
            "\"direct\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Private).expect("couldn't serialize visibility"),
            "\"private\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Unlisted).expect("couldn't serialize visibility"),
            "\"unlisted\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Public).expect("couldn't serialize visibility"),
            "\"public\"".to_string()
        );
    }

    #[test]
    fn test_serialize_status() {
        let status = StatusBuilder::new("a status");
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\"}".to_string()
        );

        let status = StatusBuilder {
            status: "a status".into(),
            language: Some(Language::Eng),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\",\"language\":\"eng\"}"
        );
    }
}

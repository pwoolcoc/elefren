//! A module containing everything relating to a account returned from the api.

/// Represents a user of Mastodon and their associated profile.
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Account {
    // Base Attributes
    id: String,
    username: String,
    acct: String,
    url: String, // TODO url::Url

    // Display Attributes
    display_name: String,
    note: String,
    avatar: String, // TODO url::Url
    avatar_static: String, // TODO url::Url
    header: String, // TODO url::Url
    header_static: String, // TODO url::Url
    locked: bool,
    #[cfg(feature = "mastodon_2_4_0")]
    emojis: Vec<Emoji>,
    #[cfg(feature = "mastodon_3_1_0")]
    discoverable: bool,

    // Statistical Attributes
    created_at: DateTime<Utc>,
    #[cfg(feature = "mastodon_3_0_0")]
    last_status_at: DateTime<Utc>,
    statuses_count: u64,
    followers_count: u64,
    following_count: u64,

    // Optional Attributes
    #[cfg(feature = "mastodon_2_1_0")]
    moved: Option<Box<Account>>,
    #[cfg(feature = "mastodon_2_4_0")]
    fields: Option<Vec<MetadataField>>,
    #[cfg(feature = "mastodon_2_4_0")]
    bot: Option<bool>,
    #[cfg(feature = "mastodon_2_4_0")]
    source: Option<Source>,
    #[cfg(feature = "mastodon_3_3_0")]
    suspended: Option<bool>,
    #[cfg(feature = "mastodon_3_3_0")]
    mute_expires_at: Option<DateTime<Utc>>,

    /// A place that unknown fields go. This is mainly provided for forwards compatibility,
    /// i.e. if you want to support mastodon versions going back to 2.4.0 but don't want deser
    /// errors with newer versions, you can set `--no-default-features --features mastodon_2_4_0`
    /// and newer fields will go here so they can still be used
    #[serde(flatten)]
    elefren_extra_fields: HashMap<String, Value>,
}
impl Account {
    ///  The account id `header`
    pub fn id(&self) -> &str {
        &self.id
    }
    ///  The username of the account, not including domain.
    pub fn username(&self) -> &str {
        &self.username
    }
    /// The Webfinger account URI. Equal to `username` for local users, or `username@domain` for remote users.
    pub fn acct(&self) -> &str {
        &self.acct
    }
    /// The location of the user's profile page.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// The profile's display name.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    /// The profile's bio / description.
    pub fn note(&self) -> &str {
        &self.note
    }
    /// An image icon that is shown next to statuses and in the profile.
    pub fn avatar(&self) -> &str {
        &self.avatar
    }
    /// A static version of the avatar. Equal to `avatar` if its value is a static image; different if `avatar` is an animated GIF.
    pub fn avatar_static(&self) -> &str {
        &self.avatar_static
    }
    /// An image banner that is shown above the profile and in profile cards.
    pub fn header(&self) -> &str {
        &self.header
    }
    /// A static version of the header. Equal to header if its value is a static image; different if header is an animated GIF.
    pub fn header_static(&self) -> &str {
        &self.header_static
    }
    /// Whether the account manually approves follow requests.
    pub fn locked(&self) -> bool {
        self.locked
    }
    #[cfg(feature = "mastodon_2_4_0")]
    /// Custom emoji entities to be used when rendering the profile. If none, an empty array will be returned.
    pub fn emojis(&self) -> &[Emoji] {
        &self.emojis
    }
    #[cfg(feature = "mastodon_3_1_0")]
    /// Whether the account has opted into discovery features such as the profile directory.
    pub fn discoverable(&self) -> bool { todo!() }
    /// When the account was created.
    pub fn created_at(&self) -> &DateTime<Utc> { todo!() }
    #[cfg(feature = "mastodon_3_0_0")]
    /// When the most recent status was posted.
    pub fn last_status_at(&self) -> &DateTime<Utc> { todo!() }
    /// How many statuses are attached to this account.
    pub fn statuses_count(&self) -> u64 { todo!() }
    /// The reported followers of this profile.
    pub fn followers_count(&self) -> u64 { todo!() }
    /// The reported follows of this profile.
    pub fn following_count(&self) -> u64 { todo!() }
    #[cfg(feature = "mastodon_2_1_0")]
    /// Indicates that the profile is currently inactive and that its user has moved to a new account.
    pub fn moved(&self) -> Option<&Box<Account>> { todo!() }
    #[cfg(feature = "mastodon_2_4_0")]
    /// Additional metadata attached to a profile as name-value pairs.
    pub fn fields(&self) -> Option<&Vec<MetadataField>> { todo!() }
    #[cfg(feature = "mastodon_2_4_0")]
    /// Boolean indicating whether this account is a bot or not
    pub fn bot(&self) -> Option<bool> { todo!() }
    #[cfg(feature = "mastodon_2_4_0")]
    /// An extra entity to be used with API methods to verify credentials and update credentials.
    pub fn source(&self) -> Option<&Source> { todo!() }
    #[cfg(feature = "mastodon_3_3_0")]
    /// An extra entity returned when an account is suspended.
    pub fn suspended(&self) -> Option<bool> { todo!() }
    #[cfg(feature = "mastodon_3_3_0")]
    /// When a timed mute will expire, if applicable.
    pub fn mute_expires_at(&self) -> Option<&DateTime<Utc>> { todo!() }
}

#[cfg(feature = "mastodon_2_4_0")]
/// Represents a profile field as a name-value pair with optional verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetadataField {
    /// name part of metadata
    pub name: String,
    /// value part of metadata
    pub value: String,
}

#[cfg(feature = "mastodon_2_4_0")]
impl MetadataField {
    /// Create a new MetadataField
    pub fn new(name: &str, value: &str) -> MetadataField {
        MetadataField {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// Represents display or publishing preferences of user's own account. Returned as an additional entity when verifying and updated credentials, as an attribute of Account.
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Source {
    /// Profile bio.
    note: String,
    #[cfg(feature = "mastodon_2_4_0")]
    /// Metadata about the account.
    fields: Vec<MetadataField>,
    /// The default post privacy to be used for new statuses.
    privacy: Option<Visibility>,
    #[serde(deserialize_with = "string_or_bool")]
    /// Whether new statuses should be marked sensitive by default.
    sensitive: Option<bool>,
    #[cfg(feature = "mastodon_2_4_2")]
    /// The default posting language for new statuses.
    language: Option<Language>,
}

fn string_or_bool<'de, D: de::Deserializer<'de>>(val: D) -> ::std::result::Result<Option<bool>, D::Error> {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(untagged)]
    pub enum BoolOrString {
        Bool(bool),
        Str(String),
    }

    Ok(Some(match BoolOrString::deserialize(val)? {
        BoolOrString::Bool(b) => b,
        BoolOrString::Str(ref s) => {
            if s == "true" {
                true
            } else if s == "false" {
                false
            } else {
                return Err(de::Error::invalid_value(
                    Unexpected::Str(s),
                    &"true or false",
                ));
            }
        },
    }))
}

/// Options for the source of the update_credentials call
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct UpdateSource {
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Visibility>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
}

/// Data for the credentials of the update_credentials call
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Credentials {
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<PathBuf>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<PathBuf>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<UpdateSource>,
    /// TODO
    #[cfg(feature = "mastodon_2_4_0")]
    #[serde(serialize_with = "fields_attributes_ser::ser")]
    pub fields_attributes: Vec<MetadataField>,
}

#[cfg(feature = "mastodon_2_4_0")]
mod fields_attributes_ser {
    use super::*;
    use serde::ser::{SerializeMap, Serializer};
    pub(crate) fn ser<S>(attrs: &[MetadataField], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(attrs.len()))?;
        for (i, field) in attrs.iter().enumerate() {
            map.serialize_entry(&i, &field)?;
        }
        map.end()
    }
}

#[cfg(feature = "mastodon_2_4_0")]
use crate::status::Emoji;
use crate::visibility::Visibility;
use chrono::prelude::*;
#[cfg(feature = "mastodon_2_4_2")]
use isolang::Language;
use serde::{
    de::{self, Unexpected},
    Deserialize,
    Serialize,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::PathBuf,
};

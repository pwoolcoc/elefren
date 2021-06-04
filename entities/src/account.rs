//! A module containing everything relating to a account returned from the api.

/// Represents a user of Mastodon and their associated profile.
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Entity)]
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
    elefren_extra: HashMap<String, Value>,
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
    pub fn discoverable(&self) -> Option<bool> {
        Some(self.discoverable)
    }
    /// When the account was created.
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    #[cfg(feature = "mastodon_3_0_0")]
    /// When the most recent status was posted.
    pub fn last_status_at(&self) -> &DateTime<Utc> {
        &self.last_status_at
    }
    /// How many statuses are attached to this account.
    pub fn statuses_count(&self) -> u64 {
        self.statuses_count
    }
    /// The reported followers of this profile.
    pub fn followers_count(&self) -> u64 {
        self.followers_count
    }
    /// The reported follows of this profile.
    pub fn following_count(&self) -> u64 {
        self.following_count
    }
    #[cfg(feature = "mastodon_2_1_0")]
    /// Indicates that the profile is currently inactive and that its user has moved to a new account.
    pub fn moved(&self) -> Option<&Box<Account>> {
        self.moved.as_ref()
    }
    #[cfg(feature = "mastodon_2_4_0")]
    /// Additional metadata attached to a profile as name-value pairs.
    pub fn fields(&self) -> Option<&Vec<MetadataField>> {
        self.fields.as_ref()
    }
    #[cfg(feature = "mastodon_2_4_0")]
    /// Boolean indicating whether this account is a bot or not
    pub fn bot(&self) -> Option<bool> {
        self.bot
    }
    #[cfg(feature = "mastodon_2_4_0")]
    /// An extra entity to be used with API methods to verify credentials and update credentials.
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }
    #[cfg(feature = "mastodon_3_3_0")]
    /// An extra entity returned when an account is suspended.
    pub fn suspended(&self) -> Option<bool> {
        self.suspended
    }
    #[cfg(feature = "mastodon_3_3_0")]
    /// When a timed mute will expire, if applicable.
    pub fn mute_expires_at(&self) -> Option<&DateTime<Utc>> {
        self.mute_expires_at.as_ref()
    }
}

#[cfg(feature = "mastodon_2_4_0")]
/// Represents a profile field as a name-value pair with optional verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Entity)]
pub struct MetadataField {
    name: String,
    value: String,
    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}

#[cfg(feature = "mastodon_2_4_0")]
impl MetadataField {
    /// Create a new MetadataField
    pub fn new(name: &str, value: &str) -> MetadataField {
        MetadataField {
            name: name.into(),
            value: value.into(),
            elefren_extra: HashMap::new(),
        }
    }
    /// name part of metadata
    pub fn name(&self) -> &str {
        &self.name
    }
    /// value part of metadata
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Represents display or publishing preferences of user's own account. Returned as an additional entity when verifying and updated credentials, as an attribute of Account.
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Entity)]
pub struct Source {
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
    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}
impl Source {
    /// Profile bio.
    pub fn note(&self) -> &str {
        &self.note
    }
    #[cfg(feature = "mastodon_2_4_0")]
    /// Metadata about the account.
    pub fn fields(&self) -> &[MetadataField] {
        &self.fields
    }
    /// The default post privacy to be used for new statuses.
    pub fn privacy(&self) -> Option<Visibility> {
        self.privacy
    }
    /// Whether new statuses should be marked sensitive by default.
    pub fn sensitive(&self) -> Option<bool> {
        self.sensitive
    }
    #[cfg(feature = "mastodon_2_4_2")]
    /// The default posting language for new statuses.
    pub fn language(&self) -> Option<Language> {
        self.language
    }
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
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct UpdateSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitive: Option<bool>,
}
impl UpdateSource {
    /// Get the privacy setting
    pub fn privacy(&self) -> Option<&Visibility> {
        self.privacy.as_ref()
    }
    /// Set the privacy setting
    pub fn set_privacy(&mut self, visibility: Visibility) {
        self.privacy = Some(visibility);
    }
    /// Get the sensitivity setting
    pub fn sensitive(&self) -> Option<bool> {
        self.sensitive
    }
    /// Set the sensitivity setting
    pub fn set_sensitive(&mut self, sensitive: bool) {
        self.sensitive = Some(sensitive);
    }
}

/// Data for the credentials of the update_credentials call
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Credentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<UpdateSource>,
    #[cfg(feature = "mastodon_2_4_0")]
    #[serde(serialize_with = "fields_attributes_ser::ser")]
    fields_attributes: Vec<MetadataField>,
}
impl Credentials {
    /// TODO
    pub fn display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }
    /// TODO
    pub fn set_display_name(&mut self, display_name: &str) {
        self.display_name = Some(display_name.to_string());
    }
    /// TODO
    pub fn note(&self) -> Option<&String> {
        self.note.as_ref()
    }
    /// TODO
    pub fn set_note(&mut self, note: &str) {
        self.note = Some(note.to_string());
    }
    /// TODO
    pub fn avatar(&self) -> Option<&PathBuf> {
        self.avatar.as_ref()
    }
    /// TODO
    pub fn set_avatar(&mut self, avatar: impl Into<PathBuf>) {
        self.avatar = Some(avatar.into());
    }
    /// TODO
    pub fn header(&self) -> Option<&PathBuf> {
        self.header.as_ref()
    }
    /// TODO
    pub fn set_header(&mut self, header: impl Into<PathBuf>) {
        self.header = Some(header.into());
    }
    /// TODO
    pub fn source(&self) -> Option<&UpdateSource> {
        self.source.as_ref()
    }
    /// TODO
    pub fn set_source(&mut self, source: UpdateSource) {
        self.source = Some(source);
    }
    /// TODO
    #[cfg(feature = "mastodon_2_4_0")]
    pub fn fields_attributes(&self) -> &[MetadataField] {
        &self.fields_attributes
    }
    /// TODO
    #[cfg(feature = "mastodon_2_4_0")]
    pub fn set_fields_attributes(&mut self, fields_attributes: Vec<MetadataField>) {
        self.fields_attributes = fields_attributes;
    }
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
use derive_entity::Entity;

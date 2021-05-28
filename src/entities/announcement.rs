/// Represents an announcement set by an administrator.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Announcement {
    id: String,
    text: String,
    published: bool,
    all_day: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    read: bool,
    reactions: Vec<AnnouncementReaction>,
    scheduled_at: Option<DateTime<Utc>>,
    starts_at: Option<DateTime<Utc>>,
    ends_at: Option<DateTime<Utc>>,
}
impl Announcement {
    /// The announcement id.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// The content of the announcement.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Whether the announcement is currently active.
    pub fn published(&self) -> bool {
        self.published
    }
    /// Whether the announcement has a start/end time.
    pub fn all_day(&self) -> bool {
        self.all_day
    }
    /// When the announcement was created.
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    /// When the announcement was last updated.
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    /// Whether the announcement has been read by the user.
    pub fn read(&self) -> bool {
        self.read
    }
    /// Emoji reactions attached to the announcement.
    pub fn reactions(&self) -> &[AnnouncementReaction] {
        &self.reactions
    }
    /// When the future announcement was scheduled.
    pub fn scheduled_at(&self) -> Option<&DateTime<Utc>> {
        self.scheduled_at.as_ref()
    }
    /// When the future announcement will start.
    pub fn starts_at(&self) -> Option<&DateTime<Utc>> {
        self.starts_at.as_ref()
    }
    /// When the future announcement will end.
    pub fn ends_at(&self) -> Option<&DateTime<Utc>> {
        self.ends_at.as_ref()
    }
}

/// Custom emoji fields for AnnouncementReaction
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnnouncementReactionCustomEmoji {
    url: String,
    static_url: String,
}
impl AnnouncementReactionCustomEmoji {
    /// A link to the custom emoji.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// A link to a non-animated version of the custom emoji.
    pub fn static_url(&self) -> &str {
        &self.static_url
    }
}

/// Represents an emoji reaction to an Announcement.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnnouncementReaction {
    name: String,
    count: u64,
    me: bool,
    #[serde(flatten)]
    emoji: Option<AnnouncementReactionCustomEmoji>,
}
impl AnnouncementReaction {
    /// The emoji used for the reaction. Either a unicode emoji, or a custom emoji's shortcode.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The total number of users who have added this reaction.
    pub fn count(&self) ->  u64 {
        self.count
    }
    /// Whether the authorized user has added this reaction to the announcement.
    pub fn me(&self) -> bool {
        self.me
    }
    /// Custom emoji attributes
    pub fn emoji(&self) -> Option<&AnnouncementReactionCustomEmoji> {
        self.emoji.as_ref()
    }
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


use serde::{Deserialize, Serialize};

/// Custom emoji fields for AnnouncementReaction
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnnouncementReactionCustomEmoji {
    /// A link to the custom emoji.
    pub url: String,
    /// A link to a non-animated version of the custom emoji.
    pub static_url: String,
}

/// Represents an emoji reaction to an Announcement.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnnouncementReaction {
    /// The emoji used for the reaction. Either a unicode emoji, or a custom emoji's shortcode.
    pub name: String,
    /// The total number of users who have added this reaction.
    pub count: u64,
    /// Whether the authorized user has added this reaction to the announcement.
    pub me: bool,
    #[serde(flatten)]
    pub emoji: Option<AnnouncementReactionCustomEmoji>,
}

/// Represents an announcement set by an administrator.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Announcement {
    /// The announcement id.
    id: String,
    /// The content of the announcement.
    text: String,
    /// Whether the announcement is currently active.
    published: bool,
    /// Whether the announcement has a start/end time.
    all_day: bool,
    /// When the announcement was created.
    created_at: String, // Datetime
    /// When the announcement was last updated.
    updated_at: String, // Datetime
    /// Whether the announcement has been read by the user.
    read: bool,
    /// Emoji reactions attached to the announcement.
    reactions: Vec<AnnouncementReaction>,
    /// When the future announcement was scheduled.
    scheduled_at: Option<String>, // Datetime
    /// When the future announcement will start.
    starts_at: Option<String>, // Datetime
    /// When the future announcement will end.
    ends_at: Option<String>, // Datetime
}
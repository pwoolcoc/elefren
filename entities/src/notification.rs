//! Module containing all info about notifications.

use super::{account::Account, status::Status};
use chrono::prelude::*;
use serde::Deserialize;

/// A struct containing info about a notification.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Notification {
    /// The notification ID.
    pub id: String,
    /// The type of notification.
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    /// The time the notification was created.
    pub created_at: DateTime<Utc>,
    /// The Account sending the notification to the user.
    pub account: Account,
    /// The Status associated with the notification, if applicable.
    pub status: Option<Status>,
}

/// The type of notification.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    /// Someone mentioned the application client in another status.
    Mention,
    /// Someone reblogged one of the application client's statuses.
    Reblog,
    /// Someone favourited one of the application client's statuses.
    Favourite,
    /// Someone followed the application client.
    Follow,
}

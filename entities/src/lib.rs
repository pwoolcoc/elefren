use serde::Deserialize;

/// Data structures for ser/de of account-related resources
pub mod account;
/// Data structures for ser/de of activity-related resources
pub mod activity;
#[cfg(feature = "mastodon_2_9_1")]
/// Data structures for ser/de of admin-related resources
pub mod admin;
#[cfg(feature = "mastodon_3_1_0")]
/// Data structures for ser/de of announcement-related resources
pub mod announcement;
/// Data structures for ser/de of application-related resources
pub mod application;
/// Data structures for ser/de of attachment-related resources
pub mod attachment;
/// Data structures for ser/de of card-related resources
pub mod card;
/// Data structures for ser/de of contetx-related resources
pub mod context;
/// Data structures for ser/de of streaming events
pub mod event;
/// Data structures for ser/de of filter-related resources
pub mod filter;
/// Data structures for ser/de of instance-related resources
pub mod instance;
/// Data structures for ser/de of list-related resources
pub mod list;
/// Data structures for ser/de of mention-related resources
pub mod mention;
/// Data structures for ser/de of notification-related resources
pub mod notification;
/// Data structures for ser/de of poll resources
pub mod poll;
/// Data structures for ser/de of push-subscription-related resources
pub mod push;
/// Data structures for ser/de of relationship-related resources
pub mod relationship;
/// Data structures for ser/de of report-related resources
pub mod report;
/// Data structures for ser/de of search-related resources
pub mod search_result;
/// Data structures for ser/de of status-related resources
pub mod status;
/// Data structures for ser/de of visibility-related resources
pub mod visibility;

/// An empty JSON object.
#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Empty {}

/// The purpose of this module is to alleviate imports of many common
/// structs by adding a glob import to the top of mastodon heavy
/// modules:
pub mod prelude {
    pub use super::{
        account::{Account, Source},
        attachment::{Attachment, MediaType},
        card::Card,
        context::Context,
        event::Event,
        filter::{Filter, FilterContext},
        instance::*,
        list::List,
        mention::Mention,
        notification::Notification,
        push::Subscription,
        relationship::Relationship,
        report::Report,
        search_result::{SearchResult, SearchResultV2},
        status::{Application, Status},
        Empty,
    };
    #[cfg(feature = "mastodon_2_4_0")]
    pub use super::status::Emoji;
}

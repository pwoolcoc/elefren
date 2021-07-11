/// Data structure for the Mastodon::directory method
pub use self::directory::DirectoryRequest;
/// Data structure for the Mastodon::add_filter method
pub use self::filter::AddFilterRequest;
/// Data structure for the Mastodon::add_push_subscription method
pub use self::push::{AddPushRequest, Keys, UpdatePushRequest};
/// Data structure for the Mastodon::statuses method
pub use self::statuses::StatusesRequest;
/// Data structure for the Mastodon::update_credentials method
pub use self::update_credentials::UpdateCredsRequest;

mod directory;
mod filter;
mod push;
mod statuses;
mod update_credentials;

mod util;

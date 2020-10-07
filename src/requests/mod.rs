/// Data structure for the MastodonClient::directory method
pub use self::directory::DirectoryRequest;
/// Data structure for the MastodonClient::add_filter method
pub use self::filter::AddFilterRequest;
/// Data structure for the MastodonClient::add_push_subscription method
pub use self::push::{AddPushRequest, Keys, UpdatePushRequest};
/// Data structure for the MastodonClient::statuses method
pub use self::statuses::StatusesRequest;
/// Data structure for the MastodonClient::update_credentials method
pub use self::update_credentials::UpdateCredsRequest;

mod directory;
mod filter;
mod push;
mod statuses;
mod update_credentials;

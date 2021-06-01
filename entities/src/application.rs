/// Represents an application that interfaces with the REST API to access accounts or post statuses.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Application {
    name: String,
    website: Option<String>, // TODO Url
    #[cfg(feature = "mastodon_2_8_0")]
    vapid_key: Option<String>,
    #[serde(flatten)]
    client_attributes: ClientAttributes,
}
impl Application {
    /// The name of your application.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The website associated with your application.
    pub fn website(&self) -> Option<&String> {
        self.website.as_ref()
    }
    /// Used for Push Streaming API. Returned with POST /api/v1/apps. Equivalent to PushSubscription#server_key
    #[cfg(feature = "mastodon_2_8_0")]
    pub fn vapid_key(&self) -> Option<&String> {
        self.vapid_key.as_ref()
    }
    /// Client ID key, to be used for obtaining OAuth tokens
    pub fn client_id(&self) -> &str {
        &self.client_attributes.client_id
    }
    /// Client secret key, to be used for obtaining OAuth tokens
    pub fn client_secret(&self) -> &str {
        &self.client_attributes.client_secret
    }
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct ClientAttributes {
    client_id: String,
    client_secret: String,
}

use serde::{Deserialize, Serialize};

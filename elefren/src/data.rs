use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Raw data about mastodon app. Save `Data` using `serde` to prevent needing
/// to authenticate on every run.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Data {
    /// Base url of instance eg. `https://mastodon.social`.
    pub base: Cow<'static, str>,
    /// The client's id given by the instance.
    pub client_id: Cow<'static, str>,
    /// The client's secret given by the instance.
    pub client_secret: Cow<'static, str>,
    /// Url to redirect back to your application from the instance signup.
    pub redirect: Cow<'static, str>,
    /// The client's access token.
    pub token: Cow<'static, str>,
}

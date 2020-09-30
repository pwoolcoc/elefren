//! # Elefren: API Wrapper around the Mastodon API.
//!
//! Most of the api is documented on [Mastodon's website](https://docs.joinmastodon.org/client/intro/)
//!
//! ```no_run
//! # extern crate elefren;
//! # fn main() {
//! #    run().unwrap();
//! # }
//! # fn run() -> elefren::Result<()> {
//! use elefren::{helpers::cli, prelude::*};
//!
//! let registration = Registration::new("https://mastodon.social")
//!     .client_name("elefren_test")
//!     .build()?;
//! let mastodon = cli::authenticate(registration)?;
//!
//! println!(
//!     "{:?}",
//!     mastodon
//!         .get_home_timeline()?
//!         .items_iter()
//!         .take(100)
//!         .collect::<Vec<_>>()
//! );
//! # Ok(())
//! # }
//! ```
//!
//! Elefren also supports Mastodon's Streaming API:
//!
//! # Example
//!
//! ```no_run
//! # extern crate elefren;
//! # use elefren::prelude::*;
//! # use std::error::Error;
//! use elefren::entities::event::Event;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # let data = Data {
//! #   base: "".into(),
//! #   client_id: "".into(),
//! #   client_secret: "".into(),
//! #   redirect: "".into(),
//! #   token: "".into(),
//! # };
//! let client = Mastodon::from(data);
//! for event in client.streaming_user()? {
//!     match event {
//!         Event::Update(ref status) => { /* .. */ },
//!         Event::Notification(ref notification) => { /* .. */ },
//!         Event::Delete(ref id) => { /* .. */ },
//!         Event::FiltersChanged => { /* .. */ },
//!     }
//! }
//! # Ok(())
//! # }
//! ```

#![deny(
    missing_docs,
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "nightly", allow(broken_intra_doc_links))]

use std::{borrow::Cow, io::BufRead, ops};

use reqwest::{Client, RequestBuilder, Response};
use tungstenite::client::AutoStream;

use crate::{entities::prelude::*, page::Page};

pub use isolang::Language;

pub use crate::{
    data::Data,
    errors::{ApiError, Error, Result},
    mastodon_client::{MastodonClient, MastodonUnauthenticated},
    media_builder::MediaBuilder,
    registration::Registration,
    requests::{
        AddFilterRequest,
        AddPushRequest,
        StatusesRequest,
        UpdateCredsRequest,
        UpdatePushRequest,
    },
    status_builder::{NewStatus, StatusBuilder},
};

/// Registering your App
pub mod apps;
/// Contains the struct that holds the client auth data
pub mod data;
/// Entities returned from the API
pub mod entities;
/// Errors
pub mod errors;
/// Collection of helpers for serializing/deserializing `Data` objects
pub mod helpers;
mod mastodon_client;
/// Constructing media attachments for a status.
pub mod media_builder;
/// Handling multiple pages of entities.
pub mod page;
/// Registering your app.
pub mod registration;
/// Requests
pub mod requests;
/// OAuth Scopes
pub mod scopes;
/// Constructing a status
pub mod status_builder;
#[macro_use]
mod macros;
/// Automatically import the things you need
pub mod prelude {
    pub use crate::{
        scopes::Scopes,
        Data,
        Mastodon,
        MastodonClient,
        NewStatus,
        Registration,
        StatusBuilder,
        StatusesRequest,
    };
}

/// Your mastodon application client, handles all requests to and from Mastodon.
#[derive(Clone, Debug)]
pub struct Mastodon {
    client: Client,
    /// Raw data about your mastodon instance.
    pub data: Data,
}

impl Mastodon {
    methods![get, post, delete,];

    fn route(&self, url: &str) -> String {
        format!("{}{}", self.base, url)
    }

    pub(crate) fn send_blocking(&self, req: RequestBuilder) -> Result<Response> {
        let request = req.bearer_auth(&self.token).build()?;
        let handle = tokio::runtime::Handle::current();
        Ok(handle.block_on(self.client.execute(request))?)
    }
}

impl From<Data> for Mastodon {
    /// Creates a mastodon instance from the data struct.
    fn from(data: Data) -> Mastodon {
        let mut builder = MastodonBuilder::new();
        builder.data(data);
        builder
            .build()
            .expect("We know `data` is present, so this should be fine")
    }
}

#[async_trait::async_trait]
impl MastodonClient for Mastodon {
    type Stream = EventReader<WebSocket>;

    paged_routes! {
        (get) favourites: "favourites" => Status,
        (get) blocks: "blocks" => Account,
        (get) domain_blocks: "domain_blocks" => String,
        (get) follow_requests: "follow_requests" => Account,
        (get) get_home_timeline: "timelines/home" => Status,
        (get) get_local_timeline: "timelines/public?local=true" => Status,
        (get) get_federated_timeline: "timelines/public?local=false" => Status,
        (get) get_emojis: "custom_emojis" => Emoji,
        (get) mutes: "mutes" => Account,
        (get) notifications: "notifications" => Notification,
        (get) reports: "reports" => Report,
        (get (q: &'a str, #[serde(skip_serializing_if = "Option::is_none")] limit: Option<u64>, following: bool,)) search_accounts: "accounts/search" => Account,
        (get) get_endorsements: "endorsements" => Account,
    }

    paged_routes_with_id! {
        (get) followers: "accounts/{}/followers" => Account,
        (get) following: "accounts/{}/following" => Account,
        (get) reblogged_by: "statuses/{}/reblogged_by" => Account,
        (get) favourited_by: "statuses/{}/favourited_by" => Account,
    }

    route! {
        (delete (domain: String,)) unblock_domain: "domain_blocks" => Empty,
        (get) instance: "instance" => Instance,
        (get) verify_credentials: "accounts/verify_credentials" => Account,
        (post (account_id: &str, status_ids: Vec<&str>, comment: String,)) report: "reports" => Report,
        (post (domain: String,)) block_domain: "domain_blocks" => Empty,
        (post (id: &str,)) authorize_follow_request: "accounts/follow_requests/authorize" => Empty,
        (post (id: &str,)) reject_follow_request: "accounts/follow_requests/reject" => Empty,
        (get  (q: &'a str, resolve: bool,)) search: "search" => SearchResult,
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post) clear_notifications: "notifications/clear" => Empty,
        (post (id: &str,)) dismiss_notification: "notifications/dismiss" => Empty,
        (get) get_push_subscription: "push/subscription" => Subscription,
        (delete) delete_push_subscription: "push/subscription" => Empty,
        (get) get_filters: "filters" => Vec<Filter>,
        (get) get_follow_suggestions: "suggestions" => Vec<Account>,
    }

    route_v2! {
        (get (q: &'a str, resolve: bool,)) search_v2: "search" => SearchResultV2,
    }

    route_id! {
        (get) get_account: "accounts/{}" => Account,
        (post) follow: "accounts/{}/follow" => Relationship,
        (post) unfollow: "accounts/{}/unfollow" => Relationship,
        (post) block: "accounts/{}/block" => Relationship,
        (post) unblock: "accounts/{}/unblock" => Relationship,
        (get) mute: "accounts/{}/mute" => Relationship,
        (get) unmute: "accounts/{}/unmute" => Relationship,
        (get) get_notification: "notifications/{}" => Notification,
        (get) get_status: "statuses/{}" => Status,
        (get) get_context: "statuses/{}/context" => Context,
        (get) get_card: "statuses/{}/card" => Card,
        (post) reblog: "statuses/{}/reblog" => Status,
        (post) unreblog: "statuses/{}/unreblog" => Status,
        (post) favourite: "statuses/{}/favourite" => Status,
        (post) unfavourite: "statuses/{}/unfavourite" => Status,
        (delete) delete_status: "statuses/{}" => Empty,
        (get) get_filter: "filters/{}" => Filter,
        (delete) delete_filter: "filters/{}" => Empty,
        (delete) delete_from_suggestions: "suggestions/{}" => Empty,
        (post) endorse_user: "accounts/{}/pin" => Relationship,
        (post) unendorse_user: "accounts/{}/unpin" => Relationship,
    }

    fn add_filter(&self, request: &mut AddFilterRequest) -> Result<Filter> {
        let url = self.route("/api/v1/filters");
        let response = self.send_blocking(self.client.post(&url).json(&request))?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status));
        } else if status.is_server_error() {
            return Err(Error::Server(status));
        }

        deserialise_blocking(response)
    }

    /// PUT /api/v1/filters/:id
    fn update_filter(&self, id: &str, request: &mut AddFilterRequest) -> Result<Filter> {
        let url = self.route(&format!("/api/v1/filters/{}", id));
        let response = self.send_blocking(self.client.put(&url).json(&request))?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status));
        } else if status.is_server_error() {
            return Err(Error::Server(status));
        }

        deserialise_blocking(response)
    }

    fn update_credentials(&self, builder: &mut UpdateCredsRequest) -> Result<Account> {
        let changes = builder.build()?;
        let url = self.route("/api/v1/accounts/update_credentials");
        let response = self.send_blocking(self.client.patch(&url).json(&changes))?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status));
        } else if status.is_server_error() {
            return Err(Error::Server(status));
        }

        deserialise_blocking(response)
    }

    /// Post a new status to the account.
    fn new_status(&self, status: NewStatus) -> Result<Status> {
        let response = self.send_blocking(
            self.client
                .post(&self.route("/api/v1/statuses"))
                .json(&status),
        )?;

        deserialise_blocking(response)
    }

    /// Get timeline filtered by a hashtag(eg. `#coffee`) either locally or
    /// federated.
    fn get_hashtag_timeline(&self, hashtag: &str, local: bool) -> Result<Page<Status>> {
        let base = "/api/v1/timelines/tag/";
        let url = if local {
            self.route(&format!("{}{}?local=1", base, hashtag))
        } else {
            self.route(&format!("{}{}", base, hashtag))
        };

        Page::new(self, self.send_blocking(self.client.get(&url))?)
    }

    /// Get statuses of a single account by id. Optionally only with pictures
    /// and or excluding replies.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use elefren::prelude::*;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from(data);
    /// let statuses = client.statuses("user-id", None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use elefren::prelude::*;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from(data);
    /// let mut request = StatusesRequest::new();
    /// request.only_media();
    /// let statuses = client.statuses("user-id", request)?;
    /// # Ok(())
    /// # }
    /// ```
    fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status>>
    where
        S: Into<Option<StatusesRequest<'a>>>,
    {
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.base, id);

        if let Some(request) = request.into() {
            url = format!("{}{}", url, request.to_querystring()?);
        }

        let response = self.send_blocking(self.client.get(&url))?;

        Page::new(self, response)
    }

    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship>> {
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += &ids[0];
        } else {
            for id in ids {
                url += "id[]=";
                url += &id;
                url += "&";
            }
            url.pop();
        }

        let response = self.send_blocking(self.client.get(&url))?;

        Page::new(self, response)
    }

    /// Add a push notifications subscription
    fn add_push_subscription(&self, request: &AddPushRequest) -> Result<Subscription> {
        let request = request.build()?;
        let response = self.send_blocking(
            self.client
                .post(&self.route("/api/v1/push/subscription"))
                .json(&request),
        )?;

        deserialise_blocking(response)
    }

    /// Update the `data` portion of the push subscription associated with this
    /// access token
    fn update_push_data(&self, request: &UpdatePushRequest) -> Result<Subscription> {
        let request = request.build();
        let response = self.send_blocking(
            self.client
                .put(&self.route("/api/v1/push/subscription"))
                .json(&request),
        )?;

        deserialise_blocking(response)
    }

    /// Get all accounts that follow the authenticated user
    fn follows_me(&self) -> Result<Page<Account>> {
        let me = self.verify_credentials()?;
        Ok(self.followers(&me.id)?)
    }

    /// Get all accounts that the authenticated user follows
    fn followed_by_me(&self) -> Result<Page<Account>> {
        let me = self.verify_credentials()?;
        Ok(self.following(&me.id)?)
    }

    /// returns events that are relevant to the authorized user, i.e. home
    /// timeline & notifications
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use elefren::prelude::*;
    /// # use std::error::Error;
    /// use elefren::entities::event::Event;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from(data);
    /// for event in client.streaming_user()? {
    ///     match event {
    ///         Event::Update(ref status) => { /* .. */ },
    ///         Event::Notification(ref notification) => { /* .. */ },
    ///         Event::Delete(ref id) => { /* .. */ },
    ///         Event::FiltersChanged => { /* .. */ },
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn streaming_user(&self) -> Result<Self::Stream> {
        let mut url: url::Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.token)
            .append_pair("stream", "user");
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }

    /// returns all public statuses
    fn streaming_public(&self) -> Result<Self::Stream> {
        let mut url: url::Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.token)
            .append_pair("stream", "public");
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }

    /// Returns all local statuses
    fn streaming_local(&self) -> Result<Self::Stream> {
        let mut url: url::Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.token)
            .append_pair("stream", "public:local");
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }

    /// Returns all public statuses for a particular hashtag
    fn streaming_public_hashtag(&self, hashtag: &str) -> Result<Self::Stream> {
        let mut url: url::Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.token)
            .append_pair("stream", "hashtag")
            .append_pair("tag", hashtag);
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }

    /// Returns all local statuses for a particular hashtag
    fn streaming_local_hashtag(&self, hashtag: &str) -> Result<Self::Stream> {
        let mut url: url::Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.token)
            .append_pair("stream", "hashtag:local")
            .append_pair("tag", hashtag);
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }

    /// Returns statuses for a list
    fn streaming_list(&self, list_id: &str) -> Result<Self::Stream> {
        let mut url: url::Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.token)
            .append_pair("stream", "list")
            .append_pair("list", list_id);
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }

    /// Returns all direct messages
    fn streaming_direct(&self) -> Result<Self::Stream> {
        let mut url: url::Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.token)
            .append_pair("stream", "direct");
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }

    /// Equivalent to /api/v1/media
    fn media(&self, media_builder: MediaBuilder) -> Result<Attachment> {
        use reqwest::multipart::{Form, Part};
        use std::{fs::File, io::Read};

        let mut f = File::open(media_builder.file.as_ref())?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        let part = Part::stream(bytes);
        let mut form_data = Form::new().part("file", part);

        if let Some(description) = media_builder.description {
            form_data = form_data.text("description", description);
        }

        if let Some(focus) = media_builder.focus {
            let string = format!("{},{}", focus.0, focus.1);
            form_data = form_data.text("focus", string);
        }

        let response = self.send_blocking(
            self.client
                .post(&self.route("/api/v1/media"))
                .multipart(form_data),
        )?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status));
        } else if status.is_server_error() {
            return Err(Error::Server(status));
        }

        deserialise_blocking(response)
    }
}

#[derive(Debug)]
/// WebSocket newtype so that EventStream can be implemented without coherency
/// issues
pub struct WebSocket(tungstenite::protocol::WebSocket<AutoStream>);

/// A type that streaming events can be read from
pub trait EventStream {
    /// Read a message from this stream
    fn read_message(&mut self) -> Result<String>;
}

impl<R: BufRead> EventStream for R {
    fn read_message(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.read_line(&mut buf)?;
        Ok(buf)
    }
}

impl EventStream for WebSocket {
    fn read_message(&mut self) -> Result<String> {
        Ok(self.0.read_message()?.into_text()?)
    }
}

#[derive(Debug)]
/// Iterator that produces events from a mastodon streaming API event stream
pub struct EventReader<R: EventStream>(R);
impl<R: EventStream> Iterator for EventReader<R> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::new();
        loop {
            if let Ok(line) = self.0.read_message() {
                let line = line.trim().to_string();
                if line.starts_with(':') || line.is_empty() {
                    continue;
                }
                lines.push(line);
                if let Ok(event) = self.make_event(&lines) {
                    lines.clear();
                    return Some(event);
                } else {
                    continue;
                }
            }
        }
    }
}

impl<R: EventStream> EventReader<R> {
    fn make_event(&self, lines: &[String]) -> Result<Event> {
        let event;
        let data;
        if let Some(event_line) = lines.iter().find(|line| line.starts_with("event:")) {
            event = event_line[6..].trim().to_string();
            data = lines
                .iter()
                .find(|line| line.starts_with("data:"))
                .map(|x| x[5..].trim().to_string());
        } else {
            use serde::Deserialize;
            #[derive(Deserialize)]
            struct Message {
                pub event: String,
                pub payload: Option<String>,
            }
            let message = serde_json::from_str::<Message>(&lines[0])?;
            event = message.event;
            data = message.payload;
        }
        let event: &str = &event;
        Ok(match event {
            "notification" => {
                let data = data.ok_or_else(|| {
                    Error::Other("Missing `data` line for notification".to_string())
                })?;
                let notification = serde_json::from_str::<Notification>(&data)?;
                Event::Notification(notification)
            },
            "update" => {
                let data =
                    data.ok_or_else(|| Error::Other("Missing `data` line for update".to_string()))?;
                let status = serde_json::from_str::<Status>(&data)?;
                Event::Update(status)
            },
            "delete" => {
                let data =
                    data.ok_or_else(|| Error::Other("Missing `data` line for delete".to_string()))?;
                Event::Delete(data)
            },
            "filters_changed" => Event::FiltersChanged,
            _ => return Err(Error::Other(format!("Unknown event `{}`", event))),
        })
    }
}

impl ops::Deref for Mastodon {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

struct MastodonBuilder {
    client: Option<Client>,
    data: Option<Data>,
}

impl MastodonBuilder {
    pub fn new() -> Self {
        MastodonBuilder {
            client: None,
            data: None,
        }
    }

    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn data(&mut self, data: Data) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Result<Mastodon> {
        Ok(if let Some(data) = self.data {
            Mastodon {
                client: self.client.unwrap_or_else(Client::new),
                data,
            }
        } else {
            return Err(Error::MissingField("missing field 'data'"));
        })
    }
}

/// Client that can make unauthenticated calls to a mastodon instance
#[derive(Clone, Debug)]
pub struct MastodonUnauth {
    client: Client,
    base: url::Url,
}

impl MastodonUnauth {
    /// Create a new unauthenticated client
    pub fn new(base: &str) -> Result<MastodonUnauth> {
        let base = if base.starts_with("https://") {
            base.to_string()
        } else {
            format!("https://{}", base)
        };
        Ok(MastodonUnauth {
            client: Client::new(),
            base: url::Url::parse(&base)?,
        })
    }
}

impl MastodonUnauth {
    fn route(&self, url: &str) -> Result<url::Url> {
        Ok(self.base.join(url)?)
    }

    fn send_blocking(&self, req: RequestBuilder) -> Result<Response> {
        let req = req.build()?;
        let handle = tokio::runtime::Handle::current();
        Ok(handle.block_on(self.client.execute(req))?)
    }

    /// Get a stream of the public timeline
    pub fn streaming_public(&self) -> Result<EventReader<WebSocket>> {
        let mut url: url::Url = self.route("/api/v1/streaming/public/local")?;
        url.query_pairs_mut().append_pair("stream", "public");
        let mut url: url::Url = reqwest::blocking::get(url.as_str())?
            .url()
            .as_str()
            .parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        let client = tungstenite::connect(url.as_str())?.0;

        Ok(EventReader(WebSocket(client)))
    }
}

impl MastodonUnauthenticated for MastodonUnauth {
    /// GET /api/v1/statuses/:id
    fn get_status(&self, id: &str) -> Result<Status> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        let response = self.send_blocking(self.client.get(route))?;
        deserialise_blocking(response)
    }

    /// GET /api/v1/statuses/:id/context
    fn get_context(&self, id: &str) -> Result<Context> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        let route = route.join("context")?;
        let response = self.send_blocking(self.client.get(route))?;
        deserialise_blocking(response)
    }

    /// GET /api/v1/statuses/:id/card
    fn get_card(&self, id: &str) -> Result<Card> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        let route = route.join("card")?;
        let response = self.send_blocking(self.client.get(route))?;
        deserialise_blocking(response)
    }
}

// Convert the HTTP response body from JSON. Pass up deserialization errors
// transparently.
fn deserialise_blocking<T: for<'de> serde::Deserialize<'de>>(response: Response) -> Result<T> {
    let handle = tokio::runtime::Handle::current();

    let bytes = handle.block_on(response.bytes())?;

    match serde_json::from_slice(&bytes) {
        Ok(t) => {
            log::debug!("{}", String::from_utf8_lossy(&bytes));
            Ok(t)
        },
        // If deserializing into the desired type fails try again to
        // see if this is an error response.
        Err(e) => {
            log::error!("{}", String::from_utf8_lossy(&bytes));
            if let Ok(error) = serde_json::from_slice(&bytes) {
                return Err(Error::Api(error));
            }
            Err(e.into())
        },
    }
}

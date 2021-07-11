use std::borrow::Cow;
use std::ops;

use crate::data::Data;
use crate::entities::Empty;
use crate::entities::account::Account;
use crate::entities::attachment::Attachment;
use crate::entities::card::Card;
use crate::entities::context::Context;
use crate::entities::filter::Filter;
use crate::entities::instance::*;
use crate::entities::notification::Notification;
use crate::entities::push::Subscription;
use crate::entities::relationship::Relationship;
use crate::entities::report::Report;
use crate::entities::search_result::SearchResult;
use crate::entities::search_result::SearchResultV2;
use crate::entities::status::Emoji;
use crate::entities::status::Status;
use crate::errors::Error;
use crate::errors::Result;
use crate::event_stream::EventReader;
use crate::event_stream::WebSocket;
use crate::media_builder::MediaBuilder;
use crate::page::Page;
use crate::requests::AddFilterRequest;
use crate::requests::AddPushRequest;
use crate::requests::StatusesRequest;
use crate::requests::UpdateCredsRequest;
use crate::requests::UpdatePushRequest;
use crate::status_builder::NewStatus;
use crate::util::deserialise_blocking;

use reqwest::Response;
use reqwest::RequestBuilder;
use reqwest::Client;

/// Your mastodon application client, handles all requests to and from Mastodon.
#[derive(Clone, Debug)]
pub struct Mastodon {
    pub(crate) client: Client,
    /// Raw data about your mastodon instance.
    pub data: Data,
}

impl Mastodon {
    fn get<T: for<'de> serde::Deserialize<'de>>(&self, url: String) -> Result<T> {
        self.send_blocking(self.client.get(&url)).and_then(deserialise_blocking)
    }

    fn post<T: for<'de> serde::Deserialize<'de>>(&self, url: String) -> Result<T> {
        self.send_blocking(self.client.post(&url)).and_then(deserialise_blocking)
    }

    fn delete<T: for<'de> serde::Deserialize<'de>>(&self, url: String) -> Result<T> {
        self.send_blocking(self.client.delete(&url)).and_then(deserialise_blocking)
    }

    fn route(&self, url: &str) -> String {
        format!("{}{}", self.base, url)
    }

    pub(crate) fn send_blocking(&self, req: RequestBuilder) -> Result<Response> {
        let request = req.bearer_auth(&self.token).build()?;
        let handle = tokio::runtime::Handle::current();
        handle
            .block_on(self.client.execute(request))
            .map_err(Error::from)
    }

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

    /// POST /api/v1/filters
    pub fn add_filter(&self, request: &mut AddFilterRequest) -> Result<Filter> {
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
    pub fn update_filter(&self, id: &str, request: &mut AddFilterRequest) -> Result<Filter> {
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

    /// Update credentials
    pub fn update_credentials(&self, builder: UpdateCredsRequest) -> Result<Account> {
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
    pub fn new_status(&self, status: NewStatus) -> Result<Status> {
        let response = self.send_blocking(
            self.client
                .post(&self.route("/api/v1/statuses"))
                .json(&status),
        )?;

        deserialise_blocking(response)
    }

    /// Get timeline filtered by a hashtag(eg. `#coffee`) either locally or
    /// federated.
    pub fn get_hashtag_timeline(&self, hashtag: &str, local: bool) -> Result<Page<Status>> {
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
    /// let request = StatusesRequest::new()
    ///     .only_media();
    /// let statuses = client.statuses("user-id", request)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status>>
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
    pub fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship>> {
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += ids[0];
        } else {
            for id in ids {
                url += "id[]=";
                url += id;
                url += "&";
            }
            url.pop();
        }

        let response = self.send_blocking(self.client.get(&url))?;

        Page::new(self, response)
    }

    /// Add a push notifications subscription
    pub fn add_push_subscription(&self, request: &AddPushRequest) -> Result<Subscription> {
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
    pub fn update_push_data(&self, request: &UpdatePushRequest) -> Result<Subscription> {
        let request = request.build();
        let response = self.send_blocking(
            self.client
                .put(&self.route("/api/v1/push/subscription"))
                .json(&request),
        )?;

        deserialise_blocking(response)
    }

    /// Get all accounts that follow the authenticated user
    pub fn follows_me(&self) -> Result<Page<Account>> {
        let me = self.verify_credentials()?;
        self.followers(&me.id)
    }

    /// Get all accounts that the authenticated user follows
    pub fn followed_by_me(&self) -> Result<Page<Account>> {
        let me = self.verify_credentials()?;
        self.following(&me.id)
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
    pub fn streaming_user(&self) -> Result<EventReader<WebSocket>> {
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
    pub fn streaming_public(&self) -> Result<EventReader<WebSocket>> {
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
    pub fn streaming_local(&self) -> Result<EventReader<WebSocket>> {
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
    pub fn streaming_public_hashtag(&self, hashtag: &str) -> Result<EventReader<WebSocket>> {
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
    pub fn streaming_local_hashtag(&self, hashtag: &str) -> Result<EventReader<WebSocket>> {
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
    pub fn streaming_list(&self, list_id: &str) -> Result<EventReader<WebSocket>> {
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
    pub fn streaming_direct(&self) -> Result<EventReader<WebSocket>> {
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
    pub fn media(&self, media_builder: MediaBuilder) -> Result<Attachment> {
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

impl From<Data> for Mastodon {
    /// Creates a mastodon instance from the data struct.
    fn from(data: Data) -> Mastodon {
        let mut builder = MastodonBuilder::default();
        builder.data(data);
        builder
            .build()
            .expect("We know `data` is present, so this should be fine")
    }
}

impl ops::Deref for Mastodon {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// Builder to build a `Mastodon` object
#[derive(Debug)]
pub struct MastodonBuilder {
    client: Option<Client>,
    data: Option<Data>,
}

impl Default for MastodonBuilder {
    fn default() -> Self {
        MastodonBuilder {
            client: None,
            data: None,
        }
    }
}

impl MastodonBuilder {

    /// Set the client for the mastodon object to be built
    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    /// Set the data for the mastodon object to be built
    pub fn data(&mut self, data: Data) -> &mut Self {
        self.data = Some(data);
        self
    }

    /// Build the `Mastodon` object
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


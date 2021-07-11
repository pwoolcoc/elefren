use crate::errors::Error;
use crate::errors::Result;
use crate::entities::context::Context;
use crate::util::deserialise_blocking;
use crate::entities::status::Status;
use crate::entities::card::Card;
use crate::event_stream::WebSocket;
use crate::event_stream::EventReader;

use reqwest::Response;
use reqwest::Client;
use reqwest::RequestBuilder;

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

    fn route(&self, url: &str) -> Result<url::Url> {
        self.base.join(url).map_err(Error::from)
    }

    fn send_blocking(&self, req: RequestBuilder) -> Result<Response> {
        let req = req.build()?;
        let handle = tokio::runtime::Handle::current();
        handle
            .block_on(self.client.execute(req))
            .map_err(Error::from)
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

    /// GET /api/v1/statuses/:id
    pub fn get_status(&self, id: &str) -> Result<Status> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        let response = self.send_blocking(self.client.get(route))?;
        deserialise_blocking(response)
    }

    /// GET /api/v1/statuses/:id/context
    pub fn get_context(&self, id: &str) -> Result<Context> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        let route = route.join("context")?;
        let response = self.send_blocking(self.client.get(route))?;
        deserialise_blocking(response)
    }

    /// GET /api/v1/statuses/:id/card
    pub fn get_card(&self, id: &str) -> Result<Card> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        let route = route.join("card")?;
        let response = self.send_blocking(self.client.get(route))?;
        deserialise_blocking(response)
    }
}


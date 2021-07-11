//! Async Mastodon Client
//!
//! # Example
//!
//! ```rust,no_run
//! use elefren::r#async::Client;
//! use url::Url;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #   smol::block_on(async {
//! let client = Client::new("https://mastodon.social")?;
//!
//! // iterate page-by-page
//! // this API isn't ideal, but one day we'll get better
//! // syntax support for iterating over streams and we can
//! // do better
//! let mut pages = client.public_timeline(None).await?;
//! while let Some(statuses) = pages.next_page().await? {
//!     for status in statuses {
//!         println!("{:?}", status);
//!     }
//! }
//! # Ok(())
//! # })
//! }
//! ```
#![allow(warnings)]
#![allow(missing_docs)]
use crate::{
    entities::{
        account::Account,
        activity::Activity,
        card::Card,
        context::Context,
        instance::Instance,
        poll::Poll,
        status::{Emoji, Status, Tag},
    },
    errors::{Error, Result},
    requests::{DirectoryRequest, StatusesRequest},
};
use http_types::{Method, Request, Response};
use std::fmt::Debug;
use url::Url;

pub use auth::Authenticate;
use auth::{OAuth, Unauthenticated};
pub use page::Page;

mod auth;
mod client;
mod page;

/// Async unauthenticated client
#[derive(Debug)]
pub struct Client<A: Debug + Authenticate> {
    base_url: Url,
    auth: A,
}
impl Client<Unauthenticated> {
    pub fn new<S: AsRef<str>>(base_url: S) -> Result<Client<Unauthenticated>> {
        let base_url = Url::parse(base_url.as_ref())?;
        Ok(Client {
            base_url,
            auth: Unauthenticated,
        })
    }
}
impl<A: Debug + Authenticate> Client<A> {
    async fn send(&self, mut req: Request) -> Result<Response> {
        self.auth.authenticate(&mut req).await?;
        Ok(client::fetch(req).await?)
    }

    /// GET /api/v1/timelines/public
    pub async fn public_timeline<'a, 'client: 'a, I: Into<Option<StatusesRequest<'a>>>>(
        &'client self,
        opts: I,
    ) -> Result<Page<'client, Status, A>> {
        let mut url = self.base_url.join("api/v1/timelines/public")?;
        if let Some(opts) = opts.into() {
            let qs = opts.to_querystring()?;
            url.set_query(Some(&qs[..]));
        };
        Ok(Page::new(Request::new(Method::Get, url), &self.auth))
    }

    /// GET /api/v1/timelines/tag/:tag
    pub async fn hashtag_timeline<'a, 'client: 'a, I: Into<Option<StatusesRequest<'a>>>>(
        &'client self,
        tag: &str,
        opts: I,
    ) -> Result<Page<'client, Status, A>> {
        let mut url = self
            .base_url
            .join(&format!("api/v1/timelines/tag/{}", tag))?;
        if let Some(opts) = opts.into() {
            let qs = opts.to_querystring()?;
            url.set_query(Some(&qs[..]));
        }
        Ok(Page::new(Request::new(Method::Get, url), &self.auth))
    }

    /// GET /api/v1/statuses/:id
    pub async fn status(&self, id: &str) -> Result<Status> {
        let url = self.base_url.join(&format!("api/v1/statuses/{}", id))?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/statuses/:id/context
    pub async fn context(&self, id: &str) -> Result<Context> {
        let url = self
            .base_url
            .join(&format!("api/v1/statuses/{}/context", id))?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/statuses/:id/card
    pub async fn card(&self, id: &str) -> Result<Card> {
        let url = self
            .base_url
            .join(&format!("api/v1/statuses/{}/card", id))?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/statuses/:id/reblogged_by
    pub async fn reblogged_by<'client>(
        &'client self,
        id: &str,
    ) -> Result<Page<'client, Account, A>> {
        let url = self
            .base_url
            .join(&format!("api/v1/statuses/{}/reblogged_by", id))?;
        Ok(Page::new(Request::new(Method::Get, url), &self.auth))
    }

    /// GET /api/v1/statuses/:id/favourited_by
    pub async fn favourited_by<'client>(
        &'client self,
        id: &str,
    ) -> Result<Page<'client, Account, A>> {
        let url = self
            .base_url
            .join(&format!("api/v1/statuses/{}/favourited_by", id))?;
        Ok(Page::new(Request::new(Method::Get, url), &self.auth))
    }

    /// GET /api/v1/accounts/:id
    pub async fn account(&self, id: &str) -> Result<Account> {
        let url = self.base_url.join(&format!("api/v1/accounts/{}", id))?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/accounts/:id/statuses
    pub async fn account_statuses<'a, 'client: 'a, I: Into<Option<StatusesRequest<'a>>>>(
        &'client self,
        id: &str,
        request: I,
    ) -> Result<Page<'client, Status, A>> {
        let mut url = self
            .base_url
            .join(&format!("api/v1/accounts/{}/statuses", id))?;
        if let Some(request) = request.into() {
            let qs = request.to_querystring()?;
            url.set_query(Some(&qs[..]));
        }
        Ok(Page::new(Request::new(Method::Get, url), &self.auth))
    }

    /// GET /api/v1/polls/:id
    pub async fn poll(&self, id: &str) -> Result<Poll> {
        let url = self.base_url.join(&format!("api/v1/polls/{}", id))?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/instance
    pub async fn instance(&self) -> Result<Instance> {
        let url = self.base_url.join("api/v1/instance")?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/instance/peers
    pub async fn peers(&self) -> Result<Vec<String>> {
        let url = self.base_url.join("api/v1/instance/peers")?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/instance/activity
    pub async fn activity(&self) -> Result<Option<Vec<Activity>>> {
        let url = self.base_url.join("api/v1/instance/activity")?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/custom_emojis
    pub async fn custom_emojis(&self) -> Result<Vec<Emoji>> {
        let url = self.base_url.join("api/v1/custom_emojis")?;
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/directory
    pub async fn directory<'a, I: Into<Option<DirectoryRequest<'a>>>>(
        &self,
        opts: I,
    ) -> Result<Vec<Account>> {
        let mut url = self.base_url.join("api/v1/directory")?;
        if let Some(opts) = opts.into() {
            let qs = opts.to_querystring()?;
            url.set_query(Some(&qs[..]));
        }
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }

    /// GET /api/v1/trends
    pub async fn trends<I: Into<Option<usize>>>(&self, limit: I) -> Result<Vec<Tag>> {
        let mut url = self.base_url.join("api/v1/trends")?;
        if let Some(limit) = limit.into() {
            url.set_query(Some(&format!("?limit={}", limit)));
        }
        let response = self.send(Request::new(Method::Get, url)).await?;
        Ok(deserialize(response).await?)
    }
}

async fn deserialize<T: serde::de::DeserializeOwned>(mut response: Response) -> Result<T> {
    let status = response.status();
    if status.is_client_error() {
        // TODO
        // return Err(Error::Client(status));
        return Err(Error::Other(String::from("4xx status code")));
    } else if status.is_server_error() {
        // TODO
        // return Err(Error::Server(status)) // TODO
        return Err(Error::Other(String::from("5xx status code")));
    } else if status.is_redirection() || status.is_informational() {
        return Err(Error::Other(String::from("3xx or 1xx status code")));
    }
    let bytes = response.body_bytes().await?;
    Ok(match serde_json::from_slice::<T>(&bytes) {
        Ok(t) => {
            log::debug!("{}", String::from_utf8_lossy(&bytes));
            t
        }
        Err(e) => {
            log::error!("{}", String::from_utf8_lossy(&bytes));
            let err = if let Ok(error) = serde_json::from_slice(&bytes) {
                Error::Api(error)
            } else {
                e.into()
            };
            return Err(err);
        }
    })
}

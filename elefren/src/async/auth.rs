//! Authentication mechanisms for async client
use async_mutex::Mutex;
use std::cell::RefCell;

use crate::{
    entities::{account::Account, card::Card, context::Context, status::Status},
    errors::{Error, Result},
    requests::StatusesRequest,
};
use http_types::{Method, Request, Response};
use hyper_old_types::header::{parsing, Link, RelationType};
use serde::Serialize;
use smol::{prelude::*, Async};
use std::net::{TcpStream, ToSocketAddrs};
use url::Url;

/// strategies for authenticating mastodon requests need to implement this trait
#[async_trait::async_trait]
pub trait Authenticate {
    async fn authenticate(&self, request: &mut Request) -> Result<()>;
}

/// The null-strategy, will only allow the client to call public API endpoints
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unauthenticated;
#[async_trait::async_trait]
impl Authenticate for Unauthenticated {
    async fn authenticate(&self, _: &mut Request) -> Result<()> {
        Ok(())
    }
}

/// Authenticates to the server via oauth
#[derive(Debug, Clone, PartialEq)]
pub struct OAuth {
    client_id: String,
    client_secret: String,
    redirect: String,
    token: String,
}
#[async_trait::async_trait]
impl Authenticate for Mutex<RefCell<Option<OAuth>>> {
    async fn authenticate(&self, _: &mut Request) -> Result<()> {
        unimplemented!()
    }
}

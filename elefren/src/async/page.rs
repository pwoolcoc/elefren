use super::{client, deserialize, Authenticate};
use crate::{
    entities::{account::Account, card::Card, context::Context, status::Status},
    errors::{Error, Result},
};
use http_types::{Method, Request, Response};
use hyper_old_types::header::{parsing, Link, RelationType};
use smol::{prelude::*, Async};
use std::{
    fmt::Debug,
    net::{TcpStream, ToSocketAddrs},
};
use url::Url;

// link header name
const LINK: &str = "link";

#[derive(Debug)]
pub struct Page<'client, T, A: Authenticate + Debug + 'client> {
    next: Option<Request>,
    prev: Option<Request>,
    auth: &'client A,
    _marker: std::marker::PhantomData<T>,
}
impl<'client, T: serde::de::DeserializeOwned, A: Authenticate + Debug + 'client>
    Page<'client, T, A>
{
    pub fn new(next: Request, auth: &'client A) -> Page<'client, T, A> {
        Page {
            next: Some(next),
            prev: None,
            auth,
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn next_page(&mut self) -> Result<Option<Vec<T>>> {
        let mut req = if let Some(next) = self.next.take() {
            next
        } else {
            return Ok(None);
        };
        Ok(self.send(req).await?)
    }

    pub async fn prev_page(&mut self) -> Result<Option<Vec<T>>> {
        let req = if let Some(prev) = self.prev.take() {
            prev
        } else {
            return Ok(None);
        };
        Ok(self.send(req).await?)
    }

    async fn send(&mut self, mut req: Request) -> Result<Option<Vec<T>>> {
        self.auth.authenticate(&mut req).await?;
        log::trace!("Request: {:?}", req);
        let response = client::fetch(req).await?;
        log::trace!("Response: {:?}", response);
        self.fill_links_from_resp(&response)?;
        let items = deserialize(response).await?;
        Ok(items)
    }

    fn fill_links_from_resp(&mut self, response: &Response) -> Result<()> {
        let (prev, next) = get_links(&response)?;
        self.prev = prev.map(|url| Request::new(Method::Get, url));
        self.next = next.map(|url| Request::new(Method::Get, url));
        Ok(())
    }
}

fn get_links(response: &Response) -> Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Some(link_header) = response.header(LINK) {
        let link_header = link_header.as_str();
        let link_header = link_header.as_bytes();
        let link_header: Link = parsing::from_raw_str(&link_header)?;
        for value in link_header.values() {
            if let Some(relations) = value.rel() {
                if relations.contains(&RelationType::Next) {
                    next = Some(Url::parse(value.link())?);
                }

                if relations.contains(&RelationType::Prev) {
                    prev = Some(Url::parse(value.link())?);
                }
            }
        }
    }

    Ok((prev, next))
}

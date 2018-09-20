use super::{deserialise, Mastodon, Result};
use entities::itemsiter::ItemsIter;
use hyper_old_types::header::{parsing, Link, RelationType};
use reqwest::{header::LINK, Response};
use serde::Deserialize;
use url::Url;

use http_send::HttpSend;

/// Represents a single page of API results
#[derive(Debug, Clone)]
pub struct Page<'a, T: for<'de> Deserialize<'de>, H: 'a + HttpSend> {
    mastodon: &'a Mastodon<H>,
    next: Option<Url>,
    prev: Option<Url>,
    /// Initial set of items
    pub initial_items: Vec<T>,
}

macro_rules! pages {
    ($($direction:ident: $fun:ident),*) => {

        $(
            doc_comment!(concat!(
                    "Method to retrieve the ", stringify!($direction), " page of results"),
            pub fn $fun(&mut self) -> Result<Option<Vec<T>>> {
                let url = match self.$direction.take() {
                    Some(s) => s,
                    None => return Ok(None),
                };

                let response = self.mastodon.send(
                    self.mastodon.client.get(url)
                )?;

                let (prev, next) = get_links(&response)?;
                self.next = next;
                self.prev = prev;

                deserialise(response)
            });
         )*
    }
}

impl<'a, T: for<'de> Deserialize<'de>, H: HttpSend> Page<'a, T, H> {
    pages! {
        next: next_page,
        prev: prev_page
    }

    pub(crate) fn new(mastodon: &'a Mastodon<H>, response: Response) -> Result<Self> {
        let (prev, next) = get_links(&response)?;
        Ok(Page {
            initial_items: deserialise(response)?,
            next,
            prev,
            mastodon,
        })
    }
}

impl<'a, T: Clone + for<'de> Deserialize<'de>, H: HttpSend> Page<'a, T, H> {
    /// Returns an iterator that provides a stream of `T`s
    ///
    /// This abstracts away the process of iterating over each item in a page,
    /// then making an http call, then iterating over each item in the new
    /// page, etc. The iterator provides a stream of `T`s, calling
    /// `self.next_page()`
    /// when necessary to get
    /// more of them, until
    /// there are no more items.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::prelude::*;
    /// # fn main() -> Result<(), Box<Error>> {
    /// #   let data = Data {
    /// #       base: "".into(),
    /// #       client_id: "".into(),
    /// #       client_secret: "".into(),
    /// #       redirect: "".into(),
    /// #       token: "".into(),
    /// #   };
    /// let mastodon = Mastodon::from(data);
    /// let req = StatusesRequest::new();
    /// let resp = mastodon.statuses("some-id", req)?;
    /// for status in resp.items_iter() {
    ///     // do something with status
    /// }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn items_iter(self) -> impl Iterator<Item = T> + 'a
    where
        T: 'a,
    {
        ItemsIter::new(self)
    }
}

fn get_links(response: &Response) -> Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Some(link_header) = response.headers().get(LINK) {
        let link_header = link_header.to_str().unwrap();
        let link_header = link_header.as_bytes();
        let link_header: Link = parsing::from_raw_str(&link_header).unwrap();
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

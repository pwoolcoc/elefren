use crate::Result;
use reqwest::{Client, Request, RequestBuilder, Response};
use std::fmt::Debug;

/// Abstracts away the process of turning an HTTP request into an HTTP response
pub trait HttpSend: Clone + Debug {
    /// Converts an HTTP request into an HTTP response
    fn execute(&self, client: &Client, request: Request) -> Result<Response>;

    /// Convenience method so that .build() doesn't have to be called at every
    /// call site
    fn send(&self, client: &Client, builder: RequestBuilder) -> Result<Response> {
        let request = builder.build()?;
        self.execute(client, request)
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HttpSender;

impl HttpSend for HttpSender {
    fn execute(&self, client: &Client, request: Request) -> Result<Response> {
        Ok(client.execute(request)?)
    }
}

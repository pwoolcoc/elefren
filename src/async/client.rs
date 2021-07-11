use crate::{
    entities::{account::Account, card::Card, context::Context, status::Status},
    errors::{Error, Result},
};
use http_types::{Method, Request, Response};
use hyper_old_types::header::{parsing, Link, RelationType};
use smol::{prelude::*, Async};
use std::net::{TcpStream, ToSocketAddrs};
use url::Url;

// taken pretty much verbatim from `smol`s example

/// Sends a request and fetches the response.
pub(super) async fn fetch(req: Request) -> Result<Response> {
    // Figure out the host and the port.
    let host = req
        .url()
        .host()
        .ok_or_else(|| String::from("No host found"))?
        .to_string();
    let port = req
        .url()
        .port_or_known_default()
        .ok_or_else(|| Error::Other(String::from("No port found")))?;

    // Connect to the host.
    let socket_addr = {
        let host = host.clone();
        smol::unblock(move || (host.as_str(), port).to_socket_addrs())
            .await?
            .next()
            .ok_or_else(|| Error::Other(String::from("No socket addr")))?
    };
    let stream = Async::<TcpStream>::connect(socket_addr).await?;

    // Send the request and wait for the response.
    let resp = match req.url().scheme() {
        "http" => async_h1::connect(stream, req).await?,
        "https" => {
            // In case of HTTPS, establish a secure TLS connection first.
            let stream = async_native_tls::connect(&host, stream).await?;
            async_h1::connect(stream, req).await?
        }
        scheme => return Err(Error::Other(format!("unsupported scheme '{}'", scheme))),
    };
    Ok(resp)
}

pub(super) async fn get(url: Url) -> Result<Response> {
    let req = Request::new(Method::Get, url);
    Ok(fetch(req).await?)
}

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

pub use isolang::Language;

pub use crate::{
    data::Data,
    errors::{ApiError, Error, Result},
    media_builder::MediaBuilder,
    registration::Registration,
    requests::{
        AddFilterRequest, AddPushRequest, StatusesRequest, UpdateCredsRequest, UpdatePushRequest,
    },
    status_builder::{NewStatus, StatusBuilder},
};

/// Registering your App
pub mod apps;
/// Async client
#[cfg(feature = "async")]
pub mod r#async;
/// Contains the struct that holds the client auth data
pub mod data;
/// Entities returned from the API
pub mod entities;
/// Errors
pub mod errors;
/// Collection of helpers for serializing/deserializing `Data` objects
pub mod helpers;
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
    pub use crate::data::Data;
    pub use crate::mastodon::Mastodon;
    pub use crate::registration::Registration;
    pub use crate::requests::StatusesRequest;
    pub use crate::scopes::Scopes;
    pub use crate::status_builder::NewStatus;
    pub use crate::status_builder::StatusBuilder;
}

mod mastodon;
pub use mastodon::*;

mod mastodon_unauth;
pub use mastodon_unauth::*;

mod event_stream;
pub use event_stream::*;

/// Internal utility functionality
mod util;

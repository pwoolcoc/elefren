use serde::Deserialize;
use std::{fmt, io::Error as IoError};
use thiserror::Error;

#[cfg(feature = "toml")]
use ::toml::de::Error as TomlDeError;
#[cfg(feature = "toml")]
use ::toml::ser::Error as TomlSerError;
#[cfg(feature = "async")]
use async_native_tls::Error as TlsError;
#[cfg(feature = "env")]
use envy::Error as EnvyError;
#[cfg(feature = "async")]
use http_types::Error as HttpTypesError;
use hyper_old_types::Error as HeaderParseError;
use reqwest::{header::ToStrError as HeaderStrError, Error as HttpError, StatusCode};
use serde_json::Error as SerdeError;
use serde_qs::Error as SerdeQsError;
use serde_urlencoded::ser::Error as UrlEncodedError;
use tungstenite::error::Error as WebSocketError;
use url::ParseError as UrlError;

/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// enum of possible errors encountered using the mastodon API.
#[derive(Error, Debug)]
pub enum Error {
    /// Error from the Mastodon API. This typically means something went
    /// wrong with your authentication or data.
    Api(#[from] ApiError),
    /// Error deserialising to json. Typically represents a breaking change in
    /// the Mastodon API
    Serde(#[from] SerdeError),
    /// Error serializing to url-encoded string
    UrlEncoded(#[from] UrlEncodedError),
    /// Error encountered in the HTTP backend while requesting a route.
    Http(#[from] HttpError),
    /// Wrapper around the `std::io::Error` struct.
    Io(#[from] IoError),
    /// Wrapper around the `url::ParseError` struct.
    Url(#[from] UrlError),
    /// Missing Client Id.
    ClientIdRequired,
    /// Missing Client Secret.
    ClientSecretRequired,
    /// Missing Access Token.
    AccessTokenRequired,
    /// Generic client error.
    Client(StatusCode),
    /// Generic server error.
    Server(StatusCode),
    /// MastodonBuilder & AppBuilder error
    MissingField(&'static str),
    #[cfg(feature = "toml")]
    /// Error serializing to toml
    TomlSer(#[from] TomlSerError),
    #[cfg(feature = "toml")]
    /// Error deserializing from toml
    TomlDe(#[from] TomlDeError),
    /// Error converting an http header to a string
    HeaderStrError(#[from] HeaderStrError),
    /// Error parsing the http Link header
    HeaderParseError(#[from] HeaderParseError),
    #[cfg(feature = "env")]
    /// Error deserializing from the environment
    Envy(#[from] EnvyError),
    /// Error serializing to a query string
    SerdeQs(#[from] SerdeQsError),
    /// WebSocket error
    WebSocket(#[from] WebSocketError),
    #[cfg(feature = "async")]
    /// http-types error
    HttpTypes(#[from] HttpTypesError),
    #[cfg(feature = "async")]
    /// TLS error
    Tls(#[from] TlsError),
    /// Other errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Error returned from the Mastodon API.
#[derive(Error, Clone, Debug, Deserialize)]
pub struct ApiError {
    /// The type of error.
    pub error: Option<String>,
    /// The description of the error.
    pub error_description: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[macro_export]
/// Used to easily create errors from strings
macro_rules! format_err {
    ( $( $arg:tt )* ) => {
        {
            use elefren::Error;
            Error::Other(format!($($arg)*))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use serde_json;
    use serde_urlencoded;
    use std::io;

    macro_rules! assert_is {
        ($err:ident, $variant:pat) => {
            assert!(match $err {
                $variant => true,
                _ => false,
            });
        };
    }

    #[test]
    fn from_http_error() {
        let err: HttpError = reqwest::blocking::get("not an actual URL").unwrap_err();
        let err: Error = Error::from(err);
        assert_is!(err, Error::Http(..));
    }

    #[test]
    fn from_io_error() {
        let err: IoError = io::Error::new(io::ErrorKind::Other, "other error");
        let err: Error = Error::from(err);
        assert_is!(err, Error::Io(..));
    }

    #[test]
    fn from_serde_error() {
        let err: SerdeError = serde_json::from_str::<()>("not valid json").unwrap_err();
        let err: Error = Error::from(err);
        assert_is!(err, Error::Serde(..));
    }

    #[test]
    fn from_url_encoded_error() {
        let err: UrlEncodedError = serde_urlencoded::ser::Error::Custom("error".into());
        let err: Error = Error::from(err);
        assert_is!(err, Error::UrlEncoded(..));
    }

    #[test]
    fn from_url_error() {
        let err: UrlError = UrlError::EmptyHost;
        let err: Error = Error::from(err);
        assert_is!(err, Error::Url(..));
    }

    #[test]
    fn from_api_error() {
        let err: ApiError = ApiError {
            error: None,
            error_description: None,
        };
        let err: Error = Error::from(err);
        assert_is!(err, Error::Api(..));
    }

    #[cfg(feature = "toml")]
    #[test]
    fn from_toml_ser_error() {
        let err: TomlSerError = TomlSerError::DateInvalid;
        let err: Error = Error::from(err);
        assert_is!(err, Error::TomlSer(..));
    }

    #[cfg(feature = "toml")]
    #[test]
    fn from_toml_de_error() {
        let err: TomlDeError = ::toml::from_str::<()>("not valid toml").unwrap_err();
        let err: Error = Error::from(err);
        assert_is!(err, Error::TomlDe(..));
    }
}

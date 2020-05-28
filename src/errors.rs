use std::{error, fmt, io::Error as IoError};

#[cfg(feature = "env")]
use envy::Error as EnvyError;
use hyper_old_types::Error as HeaderParseError;
use reqwest::{header::ToStrError as HeaderStrError, Error as HttpError, StatusCode};
use serde_json::Error as SerdeError;
use serde_qs::Error as SerdeQsError;
use serde_urlencoded::ser::Error as UrlEncodedError;
#[cfg(feature = "toml")]
use tomlcrate::de::Error as TomlDeError;
#[cfg(feature = "toml")]
use tomlcrate::ser::Error as TomlSerError;
use url::ParseError as UrlError;
use tungstenite::error::Error as WebSocketError;

/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// enum of possible errors encountered using the mastodon API.
#[derive(Debug)]
pub enum Error {
    /// Error from the Mastodon API. This typically means something went
    /// wrong with your authentication or data.
    Api(ApiError),
    /// Error deserialising to json. Typically represents a breaking change in
    /// the Mastodon API
    Serde(SerdeError),
    /// Error serializing to url-encoded string
    UrlEncoded(UrlEncodedError),
    /// Error encountered in the HTTP backend while requesting a route.
    Http(HttpError),
    /// Wrapper around the `std::io::Error` struct.
    Io(IoError),
    /// Wrapper around the `url::ParseError` struct.
    Url(UrlError),
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
    TomlSer(TomlSerError),
    #[cfg(feature = "toml")]
    /// Error deserializing from toml
    TomlDe(TomlDeError),
    /// Error converting an http header to a string
    HeaderStrError(HeaderStrError),
    /// Error parsing the http Link header
    HeaderParseError(HeaderParseError),
    #[cfg(feature = "env")]
    /// Error deserializing from the environment
    Envy(EnvyError),
    /// Error serializing to a query string
    SerdeQs(SerdeQsError),
    /// WebSocket error
    WebSocket(WebSocketError),
    /// Other errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match *self {
            Error::Api(ref e) => e,
            Error::Serde(ref e) => e,
            Error::UrlEncoded(ref e) => e,
            Error::Http(ref e) => e,
            Error::Io(ref e) => e,
            Error::Url(ref e) => e,
            #[cfg(feature = "toml")]
            Error::TomlSer(ref e) => e,
            #[cfg(feature = "toml")]
            Error::TomlDe(ref e) => e,
            Error::HeaderStrError(ref e) => e,
            Error::HeaderParseError(ref e) => e,
            #[cfg(feature = "env")]
            Error::Envy(ref e) => e,
            Error::SerdeQs(ref e) => e,
            Error::WebSocket(ref e) => e,

            Error::Client(..) | Error::Server(..) => {
                return None
            },
            Error::ClientIdRequired => return None,
            Error::ClientSecretRequired => return None,
            Error::AccessTokenRequired => return None,
            Error::MissingField(_) => return None,
            Error::Other(..) => return None,
        })
    }
}

/// Error returned from the Mastodon API.
#[derive(Clone, Debug, Deserialize)]
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

impl error::Error for ApiError {}

macro_rules! from {
    ($($(#[$met:meta])* $typ:ident, $variant:ident,)*) => {
        $(
            $(#[$met])*
            impl From<$typ> for Error {
                fn from(from: $typ) -> Self {
                    use Error::*;
                    $variant(from)
                }
            }
        )*
    }
}

from! {
    HttpError, Http,
    IoError, Io,
    SerdeError, Serde,
    UrlEncodedError, UrlEncoded,
    UrlError, Url,
    ApiError, Api,
    #[cfg(feature = "toml")] TomlSerError, TomlSer,
    #[cfg(feature = "toml")] TomlDeError, TomlDe,
    HeaderStrError, HeaderStrError,
    HeaderParseError, HeaderParseError,
    #[cfg(feature = "env")] EnvyError, Envy,
    SerdeQsError, SerdeQs,
    WebSocketError, WebSocket,
    String, Other,
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
        let err: HttpError = reqwest::get("not an actual URL").unwrap_err();
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
        use tomlcrate;
        let err: TomlDeError = tomlcrate::from_str::<()>("not valid toml").unwrap_err();
        let err: Error = Error::from(err);
        assert_is!(err, Error::TomlDe(..));
    }
}

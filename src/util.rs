use crate::errors::Error;
use crate::errors::Result;

use reqwest::Response;

// Convert the HTTP response body from JSON. Pass up deserialization errors
// transparently.
pub fn deserialise_blocking<T: for<'de> serde::Deserialize<'de>>(response: Response) -> Result<T> {
    let handle = tokio::runtime::Handle::current();

    let bytes = handle.block_on(response.bytes())?;

    match serde_json::from_slice(&bytes) {
        Ok(t) => {
            log::debug!("{}", String::from_utf8_lossy(&bytes));
            Ok(t)
        }
        // If deserializing into the desired type fails try again to
        // see if this is an error response.
        Err(e) => {
            log::error!("{}", String::from_utf8_lossy(&bytes));
            if let Ok(error) = serde_json::from_slice(&bytes) {
                return Err(Error::Api(error));
            }
            Err(e.into())
        }
    }
}


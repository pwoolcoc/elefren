use envy;

use crate::data::Data;
use crate::Result;

/// Attempts to deserialize a Data struct from the environment
pub fn from_env() -> Result<Data> {
    Ok(envy::from_env()?)
}

/// Attempts to deserialize a Data struct from the environment. All keys are
/// prefixed with the given prefix
pub fn from_env_prefixed(prefix: &str) -> Result<Data> {
    Ok(envy::prefixed(prefix).from_env()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        ops::FnOnce,
        panic::{catch_unwind, UnwindSafe},
    };

    fn withenv<F: FnOnce() -> R + UnwindSafe, R>(prefix: Option<&'static str>, test: F) -> R {
        env::set_var(makekey(prefix, "BASE"), "https://example.com");
        env::set_var(makekey(prefix, "CLIENT_ID"), "adbc01234");
        env::set_var(makekey(prefix, "CLIENT_SECRET"), "0987dcba");
        env::set_var(makekey(prefix, "REDIRECT"), "urn:ietf:wg:oauth:2.0:oob");
        env::set_var(makekey(prefix, "TOKEN"), "fedc5678");

        let result = catch_unwind(test);

        env::remove_var(makekey(prefix, "BASE"));
        env::remove_var(makekey(prefix, "CLIENT_ID"));
        env::remove_var(makekey(prefix, "CLIENT_SECRET"));
        env::remove_var(makekey(prefix, "REDIRECT"));
        env::remove_var(makekey(prefix, "TOKEN"));

        fn makekey(prefix: Option<&'static str>, key: &str) -> String {
            if let Some(prefix) = prefix {
                format!("{}{}", prefix, key)
            } else {
                key.to_string()
            }
        }

        result.expect("failed")
    }

    #[test]
    fn test_from_env_no_prefix() {
        let desered = withenv(None, || from_env()).expect("Couldn't deser");
        assert_eq!(
            desered,
            Data {
                base: "https://example.com".into(),
                client_id: "adbc01234".into(),
                client_secret: "0987dcba".into(),
                redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
                token: "fedc5678".into(),
            }
        );
    }

    #[test]
    fn test_from_env_prefixed() {
        let desered = withenv(Some("APP_"), || from_env_prefixed("APP_")).expect("Couldn't deser");
        assert_eq!(
            desered,
            Data {
                base: "https://example.com".into(),
                client_id: "adbc01234".into(),
                client_secret: "0987dcba".into(),
                redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
                token: "fedc5678".into(),
            }
        );
    }
}

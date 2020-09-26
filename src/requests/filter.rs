use crate::entities::filter::FilterContext;
use serde::Serialize;
use std::time::Duration;

/// Form used to create a filter
///
/// # Example
///
/// ```
/// # extern crate elefren;
/// # use std::error::Error;
/// use elefren::{entities::filter::FilterContext, requests::AddFilterRequest};
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let request = AddFilterRequest::new("foo", FilterContext::Home);
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AddFilterRequest {
    phrase: String,
    context: FilterContext,
    irreversible: Option<bool>,
    whole_word: Option<bool>,
    #[serde(serialize_with = "serialize_duration::ser")]
    expires_in: Option<Duration>,
}

impl AddFilterRequest {
    /// Create a new AddFilterRequest
    pub fn new(phrase: &str, context: FilterContext) -> AddFilterRequest {
        AddFilterRequest {
            phrase: phrase.to_string(),
            context,
            irreversible: None,
            whole_word: None,
            expires_in: None,
        }
    }

    /// Set `irreversible` to `true`
    pub fn irreversible(&mut self) -> &mut Self {
        self.irreversible = Some(true);
        self
    }

    /// Set `whole_word` to `true`
    pub fn whole_word(&mut self) -> &mut Self {
        self.whole_word = Some(true);
        self
    }

    /// Set `expires_in` to a duration
    pub fn expires_in(&mut self, d: Duration) -> &mut Self {
        self.expires_in = Some(d);
        self
    }
}

mod serialize_duration {
    use serde::ser::Serializer;
    use std::time::Duration;

    pub(crate) fn ser<S>(duration: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(d) = duration {
            let sec = d.as_secs();
            s.serialize_u64(sec)
        } else {
            s.serialize_none()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::time::Duration;

    #[test]
    fn test_new() {
        let request = AddFilterRequest::new("foo", FilterContext::Home);
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: None,
                whole_word: None,
                expires_in: None,
            }
        )
    }

    #[test]
    fn test_irreversible() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.irreversible();
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: Some(true),
                whole_word: None,
                expires_in: None,
            }
        )
    }

    #[test]
    fn test_whole_word() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.whole_word();
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: None,
                whole_word: Some(true),
                expires_in: None,
            }
        )
    }

    #[test]
    fn test_expires_in() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.expires_in(Duration::from_secs(300));
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: None,
                whole_word: None,
                expires_in: Some(Duration::from_secs(300)),
            }
        )
    }

    #[test]
    fn test_serialize_request() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.expires_in(Duration::from_secs(300));
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"phrase":"foo","context":"home","irreversible":null,"whole_word":null,"expires_in":300}"#
        )
    }
}

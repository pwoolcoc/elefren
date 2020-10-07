use crate::errors::Error;
use serde::Serialize;
use std::borrow::Cow;

mod bool_qs_serialize {
    use serde::Serializer;

    pub fn is_false(b: &bool) -> bool {
        !*b
    }

    pub fn serialize<S: Serializer>(b: &bool, s: S) -> Result<S::Ok, S::Error> {
        if *b {
            s.serialize_i64(1)
        } else {
            s.serialize_i64(0)
        }
    }
}

/// Represents the options for the directory request
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct DirectoryRequest<'a> {
    offset: Option<usize>,
    limit: Option<usize>,
    order: Option<Cow<'a, str>>, // TODO enum
    #[serde(skip_serializing_if = "bool_qs_serialize::is_false")]
    #[serde(serialize_with = "bool_qs_serialize::serialize")]
    local: bool,
}
impl<'a> DirectoryRequest<'a> {
    /// make a new DirectoryRequest builder
    pub fn new() -> Self {
        DirectoryRequest::default()
    }

    /// sets the offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// sets the limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// sets the order
    pub fn order<I: Into<Cow<'a, str>>>(mut self, order: I) -> Self {
        self.order = Some(order.into());
        self
    }

    /// sets the local
    pub fn local(mut self) -> Self {
        self.local = true;
        self
    }

    /// Turns this builder into a querystring
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::requests::DirectoryRequest;
    /// let request = DirectoryRequest::new();
    /// assert_eq!(
    ///     &request
    ///         .limit(10)
    ///         .to_querystring()
    ///         .expect("Couldn't serialize qs"),
    ///     "limit=10"
    /// );
    /// ```
    pub fn to_querystring(&self) -> Result<String, Error> {
        Ok(serde_qs::to_string(&self)?)
    }
}

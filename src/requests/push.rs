use crate::{
    entities::push::{add_subscription, update_data},
    errors::Result,
};
use serde::Serialize;

/// Container for the key & auth strings for an AddPushRequest
///
/// # Example
///
/// ```
/// # extern crate elefren;
/// use elefren::requests::Keys;
///
/// let keys = Keys::new("anetohias===", "oeatssah=");
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Keys {
    pub(crate) p256dh: String,
    pub(crate) auth: String,
}

impl Keys {
    /// Create the `Keys` container
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::Keys;
    ///
    /// let keys = Keys::new("anetohias===", "oeatssah=");
    /// ```
    pub fn new(p256dh: &str, auth: &str) -> Keys {
        Keys {
            p256dh: p256dh.to_string(),
            auth: auth.to_string(),
        }
    }
}

/// Builder to pass to the Mastodon::add_push_subscription method
///
/// # Example
///
/// ```no_run
/// # extern crate elefren;
/// # use elefren::{MastodonClient, Mastodon, Data};
/// # fn main() -> Result<(), elefren::Error> {
/// # let data = Data {
/// #   base: "".into(),
/// #   client_id: "".into(),
/// #   client_secret: "".into(),
/// #   redirect: "".into(),
/// #   token: "".into(),
/// # };
/// use elefren::requests::{AddPushRequest, Keys};
///
/// let client = Mastodon::from(data);
///
/// let keys = Keys::new("stahesuahoei293ise===", "tasecoa,nmeozka==");
/// let mut request = AddPushRequest::new("http://example.com/push/endpoint", &keys);
/// request.follow().reblog();
///
/// client.add_push_subscription(&request)?;
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AddPushRequest {
    endpoint: String,

    p256dh: String,
    auth: String,

    follow: Option<bool>,
    favourite: Option<bool>,
    reblog: Option<bool>,
    mention: Option<bool>,
}

impl AddPushRequest {
    /// Construct a new AddPushRequest
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::{AddPushRequest, Keys};
    /// let keys = Keys::new("abcdef===", "foobar==");
    /// let push_endpoint = "https://example.com/push/endpoint";
    /// let request = AddPushRequest::new(push_endpoint, &keys);
    /// ```
    pub fn new(endpoint: &str, keys: &Keys) -> AddPushRequest {
        AddPushRequest {
            endpoint: endpoint.to_string(),
            p256dh: keys.p256dh.clone(),
            auth: keys.auth.clone(),
            ..Default::default()
        }
    }

    /// A flag that indicates if you want follow notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::{AddPushRequest, Keys};
    /// let keys = Keys::new("abcdef===", "foobar==");
    /// let push_endpoint = "https://example.com/push/endpoint";
    /// let mut request = AddPushRequest::new(push_endpoint, &keys);
    /// request.follow();
    /// ```
    pub fn follow(&mut self) -> &mut Self {
        self.follow = Some(true);
        self
    }

    /// A flag that indicates if you want favourite notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::{AddPushRequest, Keys};
    /// let keys = Keys::new("abcdef===", "foobar==");
    /// let push_endpoint = "https://example.com/push/endpoint";
    /// let mut request = AddPushRequest::new(push_endpoint, &keys);
    /// request.favourite();
    /// ```
    pub fn favourite(&mut self) -> &mut Self {
        self.favourite = Some(true);
        self
    }

    /// A flag that indicates if you want reblog notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::{AddPushRequest, Keys};
    /// let keys = Keys::new("abcdef===", "foobar==");
    /// let push_endpoint = "https://example.com/push/endpoint";
    /// let mut request = AddPushRequest::new(push_endpoint, &keys);
    /// request.reblog();
    /// ```
    pub fn reblog(&mut self) -> &mut Self {
        self.reblog = Some(true);
        self
    }

    /// A flag that indicates if you want mention notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::{AddPushRequest, Keys};
    /// let keys = Keys::new("abcdef===", "foobar==");
    /// let push_endpoint = "https://example.com/push/endpoint";
    /// let mut request = AddPushRequest::new(push_endpoint, &keys);
    /// request.mention();
    /// ```
    pub fn mention(&mut self) -> &mut Self {
        self.mention = Some(true);
        self
    }

    fn flags_present(&self) -> bool {
        self.follow.is_some()
            || self.favourite.is_some()
            || self.reblog.is_some()
            || self.mention.is_some()
    }

    pub(crate) fn build(&self) -> Result<add_subscription::Form> {
        use crate::entities::push::{
            add_subscription::{Data, Form, Keys, Subscription},
            Alerts,
        };
        let mut form = Form {
            subscription: Subscription {
                endpoint: self.endpoint.clone(),
                keys: Keys {
                    p256dh: self.p256dh.clone(),
                    auth: self.auth.clone(),
                },
            },
            data: None,
        };
        if self.flags_present() {
            let mut alerts = Alerts::default();

            if let Some(follow) = self.follow {
                alerts.follow = Some(follow);
            }

            if let Some(favourite) = self.favourite {
                alerts.favourite = Some(favourite);
            }

            if let Some(reblog) = self.reblog {
                alerts.reblog = Some(reblog);
            }

            if let Some(mention) = self.mention {
                alerts.mention = Some(mention);
            }

            form.data = Some(Data {
                alerts: Some(alerts),
            });
        }
        Ok(form)
    }
}

/// Builder to pass to the Mastodon::update_push_data method
///
/// # Example
///
/// ```no_run
/// # extern crate elefren;
/// # use elefren::{MastodonClient, Mastodon, Data};
/// # fn main() -> Result<(), elefren::Error> {
/// # let data = Data {
/// #   base: "".into(),
/// #   client_id: "".into(),
/// #   client_secret: "".into(),
/// #   redirect: "".into(),
/// #   token: "".into(),
/// # };
/// use elefren::requests::UpdatePushRequest;
///
/// let client = Mastodon::from(data);
///
/// let mut request = UpdatePushRequest::new("foobar");
/// request.follow(true).reblog(true);
///
/// client.update_push_data(&request)?;
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct UpdatePushRequest {
    id: String,
    follow: Option<bool>,
    favourite: Option<bool>,
    reblog: Option<bool>,
    mention: Option<bool>,
}

impl UpdatePushRequest {
    /// Construct a new UpdatePushRequest
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::UpdatePushRequest;
    /// let request = UpdatePushRequest::new("some-id");
    /// ```
    pub fn new(id: &str) -> UpdatePushRequest {
        UpdatePushRequest {
            id: id.to_string(),
            ..Default::default()
        }
    }

    /// A flag that indicates if you want follow notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::UpdatePushRequest;
    /// let mut request = UpdatePushRequest::new("foobar");
    /// request.follow(true);
    /// ```
    pub fn follow(&mut self, follow: bool) -> &mut Self {
        self.follow = Some(follow);
        self
    }

    /// A flag that indicates if you want favourite notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::UpdatePushRequest;
    /// let mut request = UpdatePushRequest::new("foobar");
    /// request.favourite(true);
    /// ```
    pub fn favourite(&mut self, favourite: bool) -> &mut Self {
        self.favourite = Some(favourite);
        self
    }

    /// A flag that indicates if you want reblog notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::UpdatePushRequest;
    /// let mut request = UpdatePushRequest::new("foobar");
    /// request.reblog(true);
    /// ```
    pub fn reblog(&mut self, reblog: bool) -> &mut Self {
        self.reblog = Some(reblog);
        self
    }

    /// A flag that indicates if you want mention notifications pushed
    ///
    /// # Example
    /// ```
    /// # extern crate elefren;
    /// use elefren::requests::UpdatePushRequest;
    /// let mut request = UpdatePushRequest::new("foobar");
    /// request.mention(true);
    /// ```
    pub fn mention(&mut self, mention: bool) -> &mut Self {
        self.mention = Some(mention);
        self
    }

    fn flags_present(&self) -> bool {
        self.follow.is_some()
            || self.favourite.is_some()
            || self.reblog.is_some()
            || self.mention.is_some()
    }

    pub(crate) fn build(&self) -> update_data::Form {
        use crate::entities::push::{
            update_data::{Data, Form},
            Alerts,
        };

        let mut form = Form {
            id: self.id.clone(),
            ..Default::default()
        };

        if self.flags_present() {
            let mut alerts = Alerts::default();
            if let Some(follow) = self.follow {
                alerts.follow = Some(follow);
            }
            if let Some(favourite) = self.favourite {
                alerts.favourite = Some(favourite);
            }
            if let Some(reblog) = self.reblog {
                alerts.reblog = Some(reblog);
            }
            if let Some(mention) = self.mention {
                alerts.mention = Some(mention);
            }
            form.data = Data {
                alerts: Some(alerts),
            };
        }
        form
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::push::{add_subscription, update_data, Alerts};

    #[test]
    fn test_keys_new() {
        let keys = Keys::new("anetohias===", "oeatssah=");
        assert_eq!(
            keys,
            Keys {
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string()
            }
        );
    }

    #[test]
    fn test_add_push_request_new() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let req = AddPushRequest::new(endpoint, &keys);
        assert_eq!(
            req,
            AddPushRequest {
                endpoint: "https://example.com/push/endpoint".to_string(),
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string(),
                follow: None,
                favourite: None,
                reblog: None,
                mention: None,
            }
        );
    }
    #[test]
    fn test_add_push_request_follow() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let mut req = AddPushRequest::new(endpoint, &keys);
        req.follow();
        assert_eq!(
            req,
            AddPushRequest {
                endpoint: "https://example.com/push/endpoint".to_string(),
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string(),
                follow: Some(true),
                favourite: None,
                reblog: None,
                mention: None,
            }
        );
    }

    #[test]
    fn test_add_push_request_favourite() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let mut req = AddPushRequest::new(endpoint, &keys);
        req.favourite();
        assert_eq!(
            req,
            AddPushRequest {
                endpoint: "https://example.com/push/endpoint".to_string(),
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string(),
                follow: None,
                favourite: Some(true),
                reblog: None,
                mention: None,
            }
        );
    }
    #[test]
    fn test_add_push_request_reblog() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let mut req = AddPushRequest::new(endpoint, &keys);
        req.reblog();
        assert_eq!(
            req,
            AddPushRequest {
                endpoint: "https://example.com/push/endpoint".to_string(),
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string(),
                follow: None,
                favourite: None,
                reblog: Some(true),
                mention: None,
            }
        );
    }
    #[test]
    fn test_add_push_request_mention() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let mut req = AddPushRequest::new(endpoint, &keys);
        req.mention();
        assert_eq!(
            req,
            AddPushRequest {
                endpoint: "https://example.com/push/endpoint".to_string(),
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string(),
                follow: None,
                favourite: None,
                reblog: None,
                mention: Some(true),
            }
        );
    }
    #[test]
    fn test_add_push_request_build_no_flags() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let req = AddPushRequest::new(endpoint, &keys);
        let form = req.build().expect("Couldn't build form");
        assert_eq!(
            form,
            add_subscription::Form {
                subscription: add_subscription::Subscription {
                    endpoint: "https://example.com/push/endpoint".to_string(),
                    keys: add_subscription::Keys {
                        p256dh: "anetohias===".to_string(),
                        auth: "oeatssah=".to_string(),
                    },
                },
                data: None,
            }
        );
    }

    #[test]
    fn test_add_push_request_build() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let mut req = AddPushRequest::new(endpoint, &keys);
        req.follow().reblog();
        let form = req.build().expect("Couldn't build form");
        assert_eq!(
            form,
            add_subscription::Form {
                subscription: add_subscription::Subscription {
                    endpoint: "https://example.com/push/endpoint".to_string(),
                    keys: add_subscription::Keys {
                        p256dh: "anetohias===".to_string(),
                        auth: "oeatssah=".to_string(),
                    },
                },
                data: Some(add_subscription::Data {
                    alerts: Some(Alerts {
                        follow: Some(true),
                        favourite: None,
                        reblog: Some(true),
                        mention: None,
                    }),
                }),
            }
        );
    }

    #[test]
    fn test_update_push_request_new() {
        let req = UpdatePushRequest::new("some-id");
        assert_eq!(
            req,
            UpdatePushRequest {
                id: "some-id".to_string(),
                follow: None,
                favourite: None,
                reblog: None,
                mention: None,
            }
        );
    }

    #[test]
    fn test_update_push_request_follow() {
        let mut req = UpdatePushRequest::new("some-id");
        req.follow(true);
        assert_eq!(
            req,
            UpdatePushRequest {
                id: "some-id".to_string(),
                follow: Some(true),
                favourite: None,
                reblog: None,
                mention: None,
            }
        );
    }
    #[test]
    fn test_update_push_request_favourite() {
        let mut req = UpdatePushRequest::new("some-id");
        req.favourite(true);
        assert_eq!(
            req,
            UpdatePushRequest {
                id: "some-id".to_string(),
                follow: None,
                favourite: Some(true),
                reblog: None,
                mention: None,
            }
        );
    }
    #[test]
    fn test_update_push_request_reblog() {
        let mut req = UpdatePushRequest::new("some-id");
        req.reblog(true);
        assert_eq!(
            req,
            UpdatePushRequest {
                id: "some-id".to_string(),
                follow: None,
                favourite: None,
                reblog: Some(true),
                mention: None,
            }
        );
    }
    #[test]
    fn test_update_push_request_mention() {
        let mut req = UpdatePushRequest::new("some-id");
        req.mention(true);
        assert_eq!(
            req,
            UpdatePushRequest {
                id: "some-id".to_string(),
                follow: None,
                favourite: None,
                reblog: None,
                mention: Some(true),
            }
        );
    }
    #[test]
    fn test_update_push_request_build_no_flags() {
        let req = UpdatePushRequest::new("some-id");
        let form = req.build();
        assert_eq!(
            form,
            update_data::Form {
                id: "some-id".to_string(),
                data: update_data::Data {
                    alerts: None
                },
            }
        );
    }

    #[test]
    fn test_update_push_request_build() {
        let mut req = UpdatePushRequest::new("some-id");
        req.favourite(false);
        let form = req.build();
        assert_eq!(
            form,
            update_data::Form {
                id: "some-id".to_string(),
                data: update_data::Data {
                    alerts: Some(Alerts {
                        follow: None,
                        favourite: Some(false),
                        reblog: None,
                        mention: None,
                    }),
                },
            }
        );
    }
}

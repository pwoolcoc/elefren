use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{
    entities::account::{Credentials, MetadataField, UpdateSource},
    errors::Result,
    status_builder,
};

/// Builder to pass to the Mastodon::update_credentials method
///
/// # Example
///
/// ```no_run
/// # extern crate elefren;
/// # use elefren::Data;
/// # fn main() -> Result<(), elefren::Error> {
/// # let data = Data {
/// #   base: "".into(),
/// #   client_id: "".into(),
/// #   client_secret: "".into(),
/// #   redirect: "".into(),
/// #   token: "".into(),
/// # };
/// use elefren::{prelude::*, status_builder::Visibility, UpdateCredsRequest};
///
/// let client = Mastodon::from(data);
/// let builder = UpdateCredsRequest::new()
///     .privacy(Visibility::Unlisted);
///
/// let result = client.update_credentials(builder)?;
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UpdateCredsRequest {
    display_name: Option<String>,
    note: Option<String>,
    avatar: Option<PathBuf>,
    header: Option<PathBuf>,
    field_attributes: Vec<MetadataField>,

    // UpdateSource fields
    privacy: Option<status_builder::Visibility>,
    sensitive: Option<bool>,
}

impl UpdateCredsRequest {
    /// Create a new UpdateCredsRequest
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    /// ```
    pub fn new() -> UpdateCredsRequest {
        Default::default()
    }

    /// Set the new display_name value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.display_name("my new display name");
    /// ```
    pub fn display_name<D: Display>(mut self, name: D) -> Self {
        self.display_name = Some(name.to_string());
        self
    }

    /// Set the new note value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.note("my new note");
    /// ```
    pub fn note<D: Display>(mut self, note: D) -> Self {
        self.note = Some(note.to_string());
        self
    }

    /// Set the new avatar value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.avatar("/path/to/my/new/avatar");
    /// ```
    pub fn avatar<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref();
        let path = path.to_path_buf();
        self.avatar = Some(path);
        self
    }

    /// Set the new header value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.header("/path/to/my/new/header");
    /// ```
    pub fn header<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref();
        let path = path.to_path_buf();
        self.header = Some(path);
        self
    }

    /// Set the new privacy value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::{status_builder::Visibility, UpdateCredsRequest};
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.privacy(Visibility::Public);
    /// ```
    pub fn privacy(mut self, privacy: status_builder::Visibility) -> Self {
        self.privacy = Some(privacy);
        self
    }

    /// Set the new sensitive value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.sensitive(true);
    /// ```
    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    /// Add a metadata field
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.field_attribute("some key", "some value");
    /// ```
    pub fn field_attribute(mut self, name: &str, value: &str) -> Self {
        self.field_attributes.push(MetadataField::new(name, value));
        self
    }

    pub(crate) fn build(self) -> Result<Credentials> {
        Ok(Credentials {
            display_name: self.display_name.clone(),
            note: self.note.clone(),
            avatar: self.avatar.clone(),
            header: self.avatar.clone(),
            source: Some(UpdateSource {
                privacy: self.privacy,
                sensitive: self.sensitive,
            }),
            fields_attributes: self.field_attributes.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::account::{Credentials, MetadataField, UpdateSource},
        status_builder::Visibility,
    };

    #[test]
    fn test_update_creds_request_new() {
        let builder = UpdateCredsRequest::new();
        assert_eq!(
            builder,
            UpdateCredsRequest {
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_display_name() {
        let builder = UpdateCredsRequest::new().display_name("foo");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                display_name: Some("foo".into()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_note() {
        let builder = UpdateCredsRequest::new().note("foo");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                note: Some("foo".into()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_avatar() {
        let builder = UpdateCredsRequest::new().avatar("/path/to/avatar.png");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                avatar: Some(Path::new("/path/to/avatar.png").to_path_buf()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_header() {
        let builder = UpdateCredsRequest::new().header("/path/to/header.png");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                header: Some(Path::new("/path/to/header.png").to_path_buf()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_privacy() {
        let builder = UpdateCredsRequest::new().privacy(Visibility::Public);
        assert_eq!(
            builder,
            UpdateCredsRequest {
                privacy: Some(Visibility::Public),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_sensitive() {
        let builder = UpdateCredsRequest::new().sensitive(true);
        assert_eq!(
            builder,
            UpdateCredsRequest {
                sensitive: Some(true),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_field_attribute() {
        let builder = UpdateCredsRequest::new().field_attribute("foo", "bar");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                field_attributes: vec![MetadataField::new("foo", "bar")],
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_build() {
        let builder = UpdateCredsRequest::new().display_name("test").note("a note");
        let creds = builder.build().expect("Couldn't build Credentials");
        assert_eq!(
            creds,
            Credentials {
                display_name: Some("test".into()),
                note: Some("a note".into()),
                source: Some(UpdateSource {
                    ..Default::default()
                }),
                ..Default::default()
            }
        );
    }
}

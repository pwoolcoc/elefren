use std::borrow::Cow;

use serde::Serialize;
use std::convert::TryInto;

use crate::{
    errors::{Error, Result},
    scopes::Scopes,
};

/// Represents an application that can be registered with a mastodon instance
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct App {
    client_name: String,
    redirect_uris: String,
    scopes: Scopes,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
}

impl App {
    /// Get an AppBuilder object
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::apps::App;
    ///
    /// let mut builder = App::builder();
    /// ```
    pub fn builder<'a>() -> AppBuilder<'a> {
        AppBuilder::new()
    }

    /// Retrieve the list of scopes that apply to this App
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::Error;
    /// use elefren::{apps::App, scopes::Scopes};
    ///
    /// # fn main() -> Result<(), Error> {
    /// let mut builder = App::builder();
    /// builder.client_name("elefren-test");
    /// let app = builder.build()?;
    /// let scopes = app.scopes();
    /// assert_eq!(scopes, &Scopes::read_all());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }
}

/// Builder struct for defining your application.
/// ```
/// use elefren::apps::App;
/// use std::error::Error;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let mut builder = App::builder();
/// builder.client_name("elefren_test");
/// let app = builder.build()?;
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct AppBuilder<'a> {
    client_name: Option<Cow<'a, str>>,
    redirect_uris: Option<Cow<'a, str>>,
    scopes: Option<Scopes>,
    website: Option<Cow<'a, str>>,
}

impl<'a> AppBuilder<'a> {
    /// Creates a new AppBuilder object
    pub fn new() -> Self {
        Default::default()
    }

    /// Name of the application. Will be displayed when the user is deciding to
    /// grant permission.
    ///
    /// In order to turn this builder into an App, this needs to be provided
    pub fn client_name<I: Into<Cow<'a, str>>>(&mut self, name: I) -> &mut Self {
        self.client_name = Some(name.into());
        self
    }

    /// Where the user should be redirected after authorization
    ///
    /// If none is specified, the default is `urn:ietf:wg:oauth:2.0:oob`
    pub fn redirect_uris<I: Into<Cow<'a, str>>>(&mut self, uris: I) -> &mut Self {
        self.redirect_uris = Some(uris.into());
        self
    }

    /// Permission scope of the application.
    ///
    /// IF none is specified, the default is Scopes::read_all()
    pub fn scopes(&mut self, scopes: Scopes) -> &mut Self {
        self.scopes = Some(scopes);
        self
    }

    /// URL to the homepage of your application.
    pub fn website<I: Into<Cow<'a, str>>>(&mut self, website: I) -> &mut Self {
        self.website = Some(website.into());
        self
    }

    /// Attempts to convert this build into an `App`
    ///
    /// Will fail if no `client_name` was provided
    pub fn build(self) -> Result<App> {
        Ok(App {
            client_name: self
                .client_name
                .ok_or(Error::MissingField("client_name"))?
                .into(),
            redirect_uris: self
                .redirect_uris
                .unwrap_or_else(|| "urn:ietf:wg:oauth:2.0:oob".into())
                .into(),
            scopes: self.scopes.unwrap_or_else(Scopes::read_all),
            website: self.website.map(|s| s.into()),
        })
    }
}

impl<'a> TryInto<App> for AppBuilder<'a> {
    type Error = Error;

    fn try_into(self) -> Result<App> {
        Ok(self.build()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builder() {
        let builder = App::builder();
        assert_eq!(builder, AppBuilder::new());
    }

    #[test]
    fn test_app_scopes() {
        let mut builder = App::builder();
        builder.client_name("test").scopes(Scopes::all());
        let app = builder.build().expect("Couldn't build App");
        assert_eq!(app.scopes(), &Scopes::all());
    }

    #[test]
    fn test_app_builder_all_methods() {
        let mut builder = AppBuilder::new();
        builder.client_name("foo-test");
        builder.redirect_uris("http://example.com");
        builder.scopes(Scopes::read_all() | Scopes::write_all());
        builder.website("https://example.com");
        let app = builder.build().expect("Couldn't build App");
        assert_eq!(
            app,
            App {
                client_name: "foo-test".to_string(),
                redirect_uris: "http://example.com".to_string(),
                scopes: Scopes::read_all() | Scopes::write_all(),
                website: Some("https://example.com".to_string()),
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_app_builder_build_fails_if_no_client_name_1() {
        App::builder().build().expect("no client-name");
    }

    #[test]
    #[should_panic]
    fn test_app_builder_build_fails_if_no_client_name_2() {
        let mut builder = App::builder();
        builder
            .website("https://example.com")
            .redirect_uris("https://example.com")
            .scopes(Scopes::all());
        builder.build().expect("no client-name");
    }

    #[test]
    fn test_app_try_into_app() {
        let app = App {
            client_name: "foo-test".to_string(),
            redirect_uris: "http://example.com".to_string(),
            scopes: Scopes::all(),
            website: None,
        };
        let expected = app.clone();
        let result = app.try_into().expect("Couldn't make App into App");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_app_builder_try_into_app() {
        let mut builder = App::builder();
        builder
            .client_name("foo-test")
            .redirect_uris("http://example.com")
            .scopes(Scopes::all());
        let expected = App {
            client_name: "foo-test".to_string(),
            redirect_uris: "http://example.com".to_string(),
            scopes: Scopes::all(),
            website: None,
        };
        let result = builder
            .try_into()
            .expect("Couldn't make AppBuilder into App");
        assert_eq!(expected, result);
    }
}

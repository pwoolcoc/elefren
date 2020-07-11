macro_rules! methods {
    ($($method:ident,)+) => {
        $(
            fn $method<T: for<'de> serde::Deserialize<'de>>(&self, url: String)
            -> Result<T>
            {
                let response = self.send(
                        self.client.$method(&url)
                )?;

                deserialise(response)
            }
         )+
    };
}

macro_rules! paged_routes {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
            "Equivalent to `", stringify!($method), " /api/v1/",
            $url,
            "`\n# Errors\nIf `access_token` is not set.",
            "\n",
            "```no_run",
            "# extern crate elefren;\n",
            "# use elefren::prelude::*;\n",
            "# fn main() -> Result<(), Box<::std::error::Error>> {\n",
            "# let data = Data {\n",
            "#     base: \"https://example.com\".into(),\n",
            "#     client_id: \"taosuah\".into(),\n",
            "#     client_secret: \"htnjdiuae\".into(),\n",
            "#     redirect: \"https://example.com\".into(),\n",
            "#     token: \"tsaohueaheis\".into(),\n",
            "# };\n",
            "let client = Mastodon::from(data);\n",
            "client.", stringify!($name), "();\n",
            "#   Ok(())\n",
            "# }\n",
            "```"
            ),
            fn $name(&self) -> Result<Page<$ret, H>> {
                let url = self.route(concat!("/api/v1/", $url));
                let response = self.send(
                        self.client.$method(&url)
                )?;

                Page::new(self, response)
            }

        }

        paged_routes!{$($rest)*}
    };

    ((get ($($(#[$m:meta])* $param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `get /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."
            ),
            fn $name<'a>(&self, $($param: $typ,)*) -> Result<Page<$ret, H>> {
                use serde_urlencoded;

                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $(
                        #[$m]
                        )*
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;

                let url = format!(concat!("/api/v1/", $url, "?{}"), &qs);

                let response = self.send(
                        self.client.get(&url)
                )?;

                Page::new(self, response)
            }
        }

        paged_routes!{$($rest)*}
    };

    () => {}
}

macro_rules! route_v2 {
    ((get ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `get /api/v2/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."
            ),
            fn $name<'a>(&self, $($param: $typ,)*) -> Result<$ret> {
                use serde_urlencoded;

                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;

                let url = format!(concat!("/api/v2/", $url, "?{}"), &qs);

                Ok(self.get(self.route(&url))?)
            }
        }

        route_v2!{$($rest)*}
    };

    () => {}
}

macro_rules! route {

    ((get ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `get /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."
            ),
            fn $name<'a>(&self, $($param: $typ,)*) -> Result<$ret> {
                use serde_urlencoded;

                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;

                let url = format!(concat!("/api/v1/", $url, "?{}"), &qs);

                Ok(self.get(self.route(&url))?)
            }
        }

        route!{$($rest)*}
    };

    (($method:ident ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `", stringify!($method), " /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set.",
            ),
            fn $name(&self, $($param: $typ,)*) -> Result<$ret> {

                let form_data = json!({
                    $(
                        stringify!($param): $param,
                    )*
                });

                let response = self.send(
                        self.client.$method(&self.route(concat!("/api/v1/", $url)))
                            .json(&form_data)
                )?;

                let status = response.status().clone();

                if status.is_client_error() {
                    return Err(Error::Client(status));
                } else if status.is_server_error() {
                    return Err(Error::Server(status));
                }

                deserialise(response)
            }
        }

        route!{$($rest)*}
    };

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `", stringify!($method), " /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set.",
                "\n",
                "```no_run",
                "# extern crate elefren;\n",
                "# use elefren::prelude::*;\n",
                "# fn main() -> Result<(), Box<::std::error::Error>> {\n",
                "# let data = Data {\n",
                "#     base: \"https://example.com\".into(),\n",
                "#     client_id: \"taosuah\".into(),\n",
                "#     client_secret: \"htnjdiuae\".into(),\n",
                "#     redirect: \"https://example.com\".into(),\n",
                "#     token: \"tsaohueaheis\".into(),\n",
                "# };\n",
                "let client = Mastodon::from(data);\n",
                "client.", stringify!($name), "();\n",
                "#   Ok(())\n",
                "# }\n",
                "```"
            ),
            fn $name(&self) -> Result<$ret> {
                self.$method(self.route(concat!("/api/v1/", $url)))
            }
        }

        route!{$($rest)*}
    };

    () => {}
}

macro_rules! route_id {

    ($(($method:ident) $name:ident: $url:expr => $ret:ty,)*) => {
        $(
            doc_comment! {
                concat!(
                    "Equivalent to `", stringify!($method), " /api/v1/",
                    $url,
                    "`\n# Errors\nIf `access_token` is not set.",
                    "\n",
                    "```no_run",
                    "# extern crate elefren;\n",
                    "# use elefren::prelude::*;\n",
                    "# fn main() -> Result<(), Box<::std::error::Error>> {\n",
                    "# let data = Data {\n",
                    "#     base: \"https://example.com\".into(),\n",
                    "#     client_id: \"taosuah\".into(),\n",
                    "#     client_secret: \"htnjdiuae\".into(),\n",
                    "#     redirect: \"https://example.com\".into(),\n",
                    "#     token: \"tsaohueaheis\".into(),\n",
                    "# };\n",
                    "let client = Mastodon::from(data);\n",
                    "client.", stringify!($name), "(\"42\");\n",
                    "#   Ok(())\n",
                    "# }\n",
                    "```"
                ),
                fn $name(&self, id: &str) -> Result<$ret> {
                    self.$method(self.route(&format!(concat!("/api/v1/", $url), id)))
                }
            }
         )*
    }

}
macro_rules! paged_routes_with_id {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `", stringify!($method), " /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set.",
                "\n",
                "```no_run",
                "# extern crate elefren;\n",
                "# use elefren::prelude::*;\n",
                "# fn main() -> Result<(), Box<::std::error::Error>> {\n",
                "# let data = Data {\n",
                "#     base: \"https://example.com\".into(),\n",
                "#     client_id: \"taosuah\".into(),\n",
                "#     client_secret: \"htnjdiuae\".into(),\n",
                "#     redirect: \"https://example.com\".into(),\n",
                "#     token: \"tsaohueaheis\".into(),\n",
                "# };\n",
                "let client = Mastodon::from(data);\n",
                "client.", stringify!($name), "(\"some-id\");\n",
                "#   Ok(())\n",
                "# }\n",
                "```"
            ),
            fn $name(&self, id: &str) -> Result<Page<$ret, H>> {
                let url = self.route(&format!(concat!("/api/v1/", $url), id));
                let response = self.send(
                        self.client.$method(&url)
                )?;

                Page::new(self, response)
            }
        }

        paged_routes_with_id!{$($rest)*}
    };

    () => {}
}

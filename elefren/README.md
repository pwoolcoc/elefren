# Elefren

## A Wrapper for the Mastodon API.

[![Build Status](https://travis-ci.org/pwoolcoc/elefren.svg?branch=master)](https://travis-ci.org/pwoolcoc/elefren)
[![Build Status](https://ci.appveyor.com/api/projects/status/qeigk3nmmps52wxv?svg=true)](https://ci.appveyor.com/project/pwoolcoc/elefren)
[![Coverage Status](https://coveralls.io/repos/github/pwoolcoc/elefren/badge.svg?branch=master&service=github)](https://coveralls.io/github/pwoolcoc/elefren?branch=master)
[![crates.io](https://img.shields.io/crates/v/elefren.svg)](https://crates.io/crates/elefren)
[![Docs](https://docs.rs/elefren/badge.svg)](https://docs.rs/elefren)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/elefren.svg)](https://crates.io/crates/elefren)

[Documentation](https://docs.rs/elefren/)

A wrapper around the [API](https://github.com/tootsuite/documentation/blob/master/docs/Using-the-API/API.md#tag) for [Mastodon](https://mastodon.social/)

## Installation

To add `elefren` to your project, add the following to the
`[dependencies]` section of your `Cargo.toml`

```toml
elefren = "0.22"
```

## Example

In your `Cargo.toml`, make sure you enable the `toml` feature:

```toml
[dependencies]
elefren = { version = "0.22", features = ["toml"] }
```

```rust,no_run
// src/main.rs
extern crate elefren;

use std::error::Error;

use elefren::prelude::*;
use elefren::helpers::toml; // requires `features = ["toml"]`
use elefren::helpers::cli;

fn main() -> Result<(), Box<dyn Error>> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register()?
    };

    let you = mastodon.verify_credentials()?;

    println!("{:#?}", you);

    Ok(())
}

fn register() -> Result<Mastodon, Box<dyn Error>> {
    let registration = Registration::new("https://mastodon.social")
                                    .client_name("elefren-examples")
                                    .build()?;
    let mastodon = cli::authenticate(registration)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}
```

It also supports the [Streaming API](https://docs.joinmastodon.org/api/streaming):

```no_run
use elefren::prelude::*;
use elefren::entities::event::Event;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = Data {
      base: "".into(),
      client_id: "".into(),
      client_secret: "".into(),
      redirect: "".into(),
      token: "".into(),
    };

    let client = Mastodon::from(data);

    for event in client.streaming_user()? {
        match event {
            Event::Update(ref status) => { /* .. */ },
            Event::Notification(ref notification) => { /* .. */ },
            Event::Delete(ref id) => { /* .. */ },
            Event::FiltersChanged => { /* .. */ },
        }
    }
    Ok(())
}
```

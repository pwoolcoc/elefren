#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]

pub use elefren::prelude::*;

use std::{error::Error, io};

use elefren::helpers::cli;
#[cfg(feature = "toml")]
use elefren::helpers::toml;

#[allow(dead_code)]
#[cfg(feature = "toml")]
fn main() -> Result<(), Box<dyn Error>> {
    register()?;
    Ok(())
}

#[allow(dead_code)]
#[cfg(feature = "toml")]
pub fn get_mastodon_data() -> Result<Mastodon, Box<dyn Error>> {
    if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Ok(Mastodon::from(data))
    } else {
        register()
    }
}

#[cfg(feature = "toml")]
pub fn register() -> Result<Mastodon, Box<dyn Error>> {
    let website = read_line("Please enter your mastodon instance url:")?;
    let registration = Registration::new(website.trim())
        .client_name("elefren-examples")
        .scopes(Scopes::all())
        .website("https://github.com/pwoolcoc/elefren")
        .build()?;
    let mastodon = cli::authenticate(registration)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}

#[cfg(feature = "toml")]
pub fn read_line(message: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

#[cfg(not(feature = "toml"))]
fn main() {}

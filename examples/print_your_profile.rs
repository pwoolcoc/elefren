#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
#[macro_use]
extern crate pretty_env_logger;
extern crate elefren;
mod register;

use crate::register::MastodonClient;
use std::error;

#[cfg(feature = "toml")]
fn main() -> Result<(), Box<dyn error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let you = mastodon.verify_credentials()?;

    println!("{:#?}", you);

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example print_your_profile --features toml\n"
    );
}

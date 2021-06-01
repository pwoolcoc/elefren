#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use crate::register::MastodonClient;
use std::error;

#[cfg(feature = "toml")]
fn main() -> Result<(), Box<dyn error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the account id you'd like to follow: ")?;
    let new_follow = mastodon.follow(input.trim())?;

    println!("{:#?}", new_follow);
    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example follow_profile --features toml\n"
    );
}

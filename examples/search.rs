#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use register::MastodonClient;
use std::error;

#[cfg(feature = "toml")]
fn main() -> Result<(), Box<error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the term you'd like to search: ")?;
    let result = mastodon.search_accounts(&input, None, true)?;

    println!("{:#?}", result.initial_items);

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("examples require the `toml` feature, run this command for this example:\n\ncargo run --example search --features toml\n");
}

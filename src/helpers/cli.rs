use std::io::{self, BufRead, Write};

use crate::{errors::Result, http_send::HttpSend, registration::Registered, Mastodon};

/// Finishes the authentication process for the given `Registered` object,
/// using the command-line
pub fn authenticate<H: HttpSend>(registration: Registered<H>) -> Result<Mastodon<H>> {
    let url = registration.authorize_url()?;

    let stdout = io::stdout();
    let stdin = io::stdin();

    let mut stdout = stdout.lock();
    let mut stdin = stdin.lock();

    writeln!(&mut stdout, "Click this link to authorize: {}", url)?;
    write!(&mut stdout, "Paste the returned authorization code: ")?;
    stdout.flush()?;

    let mut input = String::new();
    stdin.read_line(&mut input)?;
    let code = input.trim();
    Ok(registration.complete(code)?)
}

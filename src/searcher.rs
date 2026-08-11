use anyhow::Result;
use reqwest::StatusCode;
use std::{borrow::Cow, time::Duration};

use crate::parser::{Headers, Site};

// TODO: set up cli for this later using clap
static USERNAME: &str = "preetham";

#[derive(Debug)]
pub struct Response {
    url: String,
    status: StatusCode,
    time_it_took: Duration,
}

// fucking checks the fucking html if fucking found or not
pub async fn fetch_url(site: &'static Site) -> Result<Response> {
    // strip naughty characters
    let username: Cow<'_, str> = match &site.strip_bad_char {
        Some(naughty) if USERNAME.contains(naughty) => Cow::Owned(USERNAME.replace(naughty, "")),
        _ => Cow::Borrowed(USERNAME),
    };

    // meowtube.com/ -> meowtube.com/danny
    let url = site.uri_check.replace("{account}", USERNAME).trim();

    // req headers cuz fucking browser mf don't wanna think we're botz
    if let Some(ref s) = site.post_body {
        // POST
        let body = s.replace("{account}", &username);
    } else {
        // GET
    }

    todo!()
}

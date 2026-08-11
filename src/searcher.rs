use anyhow::Result;
use reqwest::StatusCode;
use std::{borrow::Cow, time::Duration};

use crate::parser::Site;

// TODO: set up cli for this later using clap
static USERNAME: &str = "preetham";

#[derive(Debug)]
pub struct Response {
    url: String,
    status: StatusCode,
    time_it_took: Duration,
}

// fucking checks the fucking html if fucking found or not
pub async fn fetch_url(client: reqwest::Client, site: &'static Site) -> Result<Response> {
    // strip naughty characters
    let username = match &site.strip_bad_char {
        Some(naughty) if USERNAME.contains(naughty) => Cow::Owned(USERNAME.replace(naughty, "")),
        _ => Cow::Borrowed(USERNAME),
    };

    // meowtube.com/ -> meowtube.com/danny
    let url = site.uri_check.replace("{account}", USERNAME);
    let url = url.trim();

    let headers = headers_maker(&site);

    let req = match &site.post_body {
        // POST
        Some(s) => {
            let body = s.replace("{account}", &username);
            client.post(url).body(body)
        }
        // GET
        None => client.get(url),
    };

    let resp = req.headers(headers).send().await?;

    todo!()
}

use reqwest::header::{
    ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONTENT_TYPE, COOKIE, HOST, HeaderMap, HeaderName,
    HeaderValue, ORIGIN, REFERER, USER_AGENT,
};

// TODO: make this compile time later instead of building the headermap every single time
// req headers cuz fucking browser mf don't wanna think we're botz
fn headers_maker(site: &'static Site) -> HeaderMap {
    let mut final_headers = HeaderMap::new();

    if let Some(headers) = &site.headers {
        let pairs = [
            (USER_AGENT, &headers.user_agent),
            (ACCEPT_LANGUAGE, &headers.accept_language),
            (CONTENT_TYPE, &headers.content_type),
            (CONTENT_TYPE, &headers.content_type2),
            (COOKIE, &headers.cookie),
            (ACCEPT, &headers.accept),
            (ACCEPT, &headers.accept2),
            (CACHE_CONTROL, &headers.cache_control),
            (HOST, &headers.host),
            (ORIGIN, &headers.origin),
            (REFERER, &headers.referer),
            (HeaderName::from_static("x-tool"), &headers.x_tool),
            (
                HeaderName::from_static("x-vt-anti-abuse-header"),
                &headers.x_vt_anti_abuse_header,
            ),
            (HeaderName::from_static("te"), &headers.te),
        ];

        final_headers.extend(pairs.into_iter().filter_map(|(name, val_opt)| {
            let val = val_opt.as_ref()?;
            let h_val = HeaderValue::from_str(val).ok()?;
            Some((name, h_val))
        }));
    }

    final_headers
}

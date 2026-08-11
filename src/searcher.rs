use crate::parser::Site;

use anyhow::Result;
use reqwest::header::{
    ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONTENT_TYPE, COOKIE, HOST, HeaderMap, HeaderName,
    HeaderValue, ORIGIN, REFERER, USER_AGENT,
};
use std::borrow::Cow;

// TODO: set up cli for this later using clap
static USERNAME: &str = "danny";

#[derive(PartialEq)]
pub enum FetchRes {
    Found,
    NotFound,
    Unknown,
}
use tokio::time::Instant;

// fucking checks the fucking html if fucking found or not
pub async fn fetch_url(client: reqwest::Client, site: &'static Site) -> Result<FetchRes> {
    // strip naughty characters
    let start = Instant::now();
    let username = match &site.strip_bad_char {
        Some(naughty) if USERNAME.contains(naughty) => Cow::Owned(USERNAME.replace(naughty, "")),
        _ => Cow::Borrowed(USERNAME),
    };

    // meowtube.com/ -> meowtube.com/danny
    let url = site.uri_check.replace("{account}", &username);
    let url = url.trim();

    let headers = headers_maker(site);

    let req = match &site.post_body {
        // POST
        Some(s) => client.post(url).body(s.replace("{account}", &username)),
        // GET
        None => client.get(url),
    };

    let resp = req.headers(headers).send().await?;
    let status = resp.status();

    if status != site.e_code && status != site.m_code {
        return Ok(FetchRes::Unknown);
    }

    let body = resp.text().await?;

    let result: FetchRes = if status == site.e_code && body.contains(&site.e_string) {
        FetchRes::Found
    } else if status == site.m_code && body.contains(&site.m_string) {
        FetchRes::NotFound
    } else {
        FetchRes::Unknown
    };

    if result == FetchRes::Found {
        let final_url = site
            .uri_pretty
            .as_ref()
            .map(|text| text.replace("{account}", &username))
            .unwrap_or_else(|| url.to_string());

        println!("{}: {}, {:?}", site.name, final_url, start.elapsed());
    }

    Ok(result)
}

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
            // the below fuckers are everywhere
            (HeaderName::from_static("x-tool"), &headers.x_tool),
            (
                HeaderName::from_static("x-vt-anti-abuse-header"),
                &headers.x_vt_anti_abuse_header,
            ),
            (HeaderName::from_static("te"), &headers.te),
        ];

        final_headers.extend(pairs.into_iter().filter_map(|(name, v)| {
            let val = v.as_ref()?;
            let h_val = HeaderValue::from_str(val).ok()?;
            Some((name, h_val))
        }));
    }

    final_headers
}

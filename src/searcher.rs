use crate::parser::Site;

use anyhow::Result;
use bstr::ByteSlice;
use futures_util::StreamExt;
use reqwest::header::{
    ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONTENT_TYPE, COOKIE, HOST, HeaderMap, HeaderName,
    HeaderValue, ORIGIN, REFERER, USER_AGENT,
};
use std::{borrow::Cow, time::Duration};

// TODO: set up cli for this later using clap
static USERNAME: &str = "danny";

#[derive(PartialEq)]
pub enum FetchRes {
    Found {
        name: String,
        url: String,
        elapsed: Duration,
    },
    NotFound,
    Unknown,
}
use tokio::time::Instant;

// fucking checks the fucking html if fucking found or not
#[inline(always)]
pub async fn fetch_url(client: reqwest::Client, site: &'static Site) -> Result<FetchRes> {
    let start = Instant::now();

    // strip naughty characters
    let username = match &site.strip_bad_char {
        Some(naughty) if USERNAME.contains(naughty) => Cow::Owned(USERNAME.replace(naughty, "")),
        _ => Cow::Borrowed(USERNAME),
    };

    // meowtube.com/ -> meowtube.com/danny
    let url = site.uri_check.replace("{account}", &username);
    let url = url.trim();

    let headers = headers_maker(site);

    let req = site.post_body.as_ref().map_or_else(
        // GET
        || client.get(url),
        // POST
        |s| client.post(url).body(s.replace("{account}", &username)),
    );

    let resp = req.headers(headers).send().await?;
    let status = resp.status();

    if status != site.e_code && status != site.m_code {
        return Ok(FetchRes::Unknown);
    }

    let mut stream = resp.bytes_stream();
    let mut buffer: Vec<u8> = Vec::new();

    let pattern = if status == site.e_code {
        &site.e_string
    } else {
        &site.m_string
    };

    let keep_tail: usize = pattern.len().saturating_sub(1);

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buffer.extend_from_slice(chunk.as_ref());

        if status == site.e_code && buffer.contains_str(&site.e_string) {
            // prettify the fucking url, instead of scummy api req, ppl wud be mad
            let final_url = site.uri_pretty.as_ref().as_ref().map_or_else(
                || url.to_string(),
                |text| text.replace("{account}", &username),
            );

            return Ok(FetchRes::Found {
                name: site.name.clone(),
                url: final_url,
                elapsed: start.elapsed(),
            });
        }

        if status == site.m_code && buffer.contains_str(&site.m_string) {
            return Ok(FetchRes::NotFound);
        }

        if buffer.len() > keep_tail * 2 {
            let excess = buffer.len() - keep_tail;
            buffer.drain(0..excess);
        }
    }

    Ok(FetchRes::Unknown)
}

// TODO: make this compile time later instead of building the headermap every single time
// req headers cuz fucking browser mf don't wanna think we're botz
#[inline(always)]
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

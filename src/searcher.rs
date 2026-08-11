use anyhow::Result;
use reqwest::StatusCode;
use std::time::Duration;

use crate::parser::Site;

#[derive(Debug)]
pub struct Response {
    url: String,
    status: StatusCode,
    time_it_took: Duration,
}

// fucking checks the fucking html if fucking found or not
pub async fn fetch_url(site: &'static Site) -> Result<Response> {
    todo!()
}

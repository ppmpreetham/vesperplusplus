// fucking checks the fucking html if fucking found or not
use anyhow::Result;
use reqwest::StatusCode;
use std::time::Duration;

#[derive(Debug)]
struct Response {
    url: String,
    status: StatusCode,
    time_it_took: Duration,
}

async fn fetch_url(url: &str) -> Result<Response> {
    todo!()
}

use crate::fetcher::fetch;

mod fetcher;
pub mod parser;
mod searcher;

#[tokio::main]
async fn main() {
    fetch().await;
}

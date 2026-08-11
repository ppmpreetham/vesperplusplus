use crate::fetcher::fetch;

mod fetcher;
pub mod parser;
mod searcher;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    fetch().await;
}

use crate::cli::run;

mod cli;
mod config;
mod fetcher;
pub mod parser;
mod searcher;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    run().await;
}

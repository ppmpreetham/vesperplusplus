use crate::parser::get_data;
use crate::searcher::fetch_url;
use reqwest::Client;
use std::time::Instant;
use tokio::task::JoinSet;

// fucking goes to every fucking website and fucking runs the fucking check function
pub async fn fetch() {
    let start = Instant::now();

    let mut set = JoinSet::new();
    let data = get_data();
    let client = Client::new();

    data.sites.iter().for_each(|site| {
        let worker = client.clone();
        set.spawn(async move {
            // TODO: do things with the res
            fetch_url(worker, site).await;
        });
    });

    let end = start.elapsed();
}

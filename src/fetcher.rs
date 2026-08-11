use crate::parser::get_data;
use crate::searcher::FetchRes;
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

    for site in &data.sites {
        let worker = client.clone();
        set.spawn(async move { fetch_url(worker, site).await });
    }

    let mut success: u32 = 0;
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(FetchRes::Found)) = res {
            success += 1;
        }
    }

    let time_taken = start.elapsed();
    println!("About {} results ({:?})", success, time_taken);
}

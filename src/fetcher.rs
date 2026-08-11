use crate::parser::get_data;
use crate::searcher::FetchRes;
use crate::searcher::fetch_url;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

// fucking goes to every fucking website and fucking runs the fucking check function
pub async fn fetch() {
    let start = Instant::now();
    let data = get_data();
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .hickory_dns(true)
        .build()
        .expect("Can't build a HTTP client");

    let sem = Arc::new(Semaphore::new(200));
    let mut set = JoinSet::new();

    for site in &data.sites {
        let worker = client.clone();
        let permit = sem.clone();
        set.spawn(async move {
            let _permit = permit.acquire_owned().await.unwrap();
            fetch_url(worker, site).await
        });
    }

    let mut success: u32 = 0;
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(FetchRes::Found)) = res {
            success += 1;
        }
    }

    println!("About {} results ({:?})", success, start.elapsed());
}

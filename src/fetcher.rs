use crate::config::get_username;
use crate::parser::get_data;
use crate::searcher::FetchRes;
use crate::searcher::fetch_url;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Semaphore;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;

pub enum FetchMode {
    Cli,
    Api { tx: Sender<FetchRes> },
}

// fucking goes to every fucking website and fucking checks if the user fucking exists
async fn fetch_core(mode: FetchMode, username: Arc<str>) -> Result<(), reqwest::Error> {
    use kdam::{BarExt, tqdm};

    let start = Instant::now();
    let data = get_data();

    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .hickory_dns(true)
        .build()?;

    let sem = Arc::new(Semaphore::new(200));
    let mut set = JoinSet::new();

    for site in &data.sites {
        let worker = client.clone();
        let permit = sem.clone();

        let task_username = username.clone();
        set.spawn(async move {
            let _permit = permit.acquire_owned().await.unwrap();
            fetch_url(worker, site, &task_username).await
        });
    }

    // api mode or cli mode
    match mode {
        FetchMode::Cli => {
            let mut pb = tqdm!(total = data.sites.len());
            let mut success: u32 = 0;

            while let Some(res) = set.join_next().await {
                if let Ok(Ok(FetchRes::Found { name, url, elapsed })) = res {
                    success += 1;
                    let _ = pb.write(format!("{name}: {url}, {elapsed:?}"));
                }
                let _ = pb.update(1);
            }

            let _ = pb.refresh();
            println!();
            println!("About {} results ({:?})", success, start.elapsed());
        }
        FetchMode::Api { tx } => {
            while let Some(res) = set.join_next().await {
                if let Ok(Ok(FetchRes::Found { name, url, elapsed })) = res {
                    let _ = tx.send(FetchRes::Found { name, url, elapsed }).await;
                }
            }
        }
    }

    Ok(())
}

pub async fn fetch_cli() {
    let username: Arc<str> = Arc::from(get_username());
    fetch_core(FetchMode::Cli, username)
        .await
        .expect("Can't build a HTTP client");
}

pub async fn fetch_api(
    username: Arc<str>,
    tx: tokio::sync::mpsc::Sender<FetchRes>,
) -> Result<(), reqwest::Error> {
    fetch_core(FetchMode::Api { tx }, username).await
}

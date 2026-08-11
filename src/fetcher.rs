use crate::parser::get_data;
use crate::searcher::fetch_url;
use std::time::Instant;
use tokio::task::JoinSet;

// fucking goes to every fucking website and fucking runs the fucking check function
async fn fetch() {
    let start = Instant::now();

    let mut set = JoinSet::new();
    let data = get_data();

    data.sites.iter().for_each(|site| {
        set.spawn(async move {
            // TODO: do things with the res
            fetch_url(site).await;
        });
    });

    let end = start.elapsed();
}

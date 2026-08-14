use crate::fetcher::fetch_api;
use crate::searcher::FetchRes;

use axum::{
    extract::Query,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Deserialize)]
pub struct UsernameQuery {
    pub username: String,
    // TODO: add filters and others later here
}

#[derive(Debug, Serialize)]
struct ApiEvent {
    name: String,
    url: String,
}

pub async fn sse_handler(
    Query(query): Query<UsernameQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let username: Arc<str> = Arc::from(query.username);
    let (tx, rx) = mpsc::channel(256);
    tokio::spawn(fetch_api(username, tx));

    let stream = ReceiverStream::new(rx).filter_map(|result| async {
        let FetchRes::Found { name, url, .. } = result else {
            return None;
        };

        Some(Ok(Event::default()
            .event("found")
            .json_data(ApiEvent { name, url })
            .expect("ApiEvent serialization cannot fail")))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

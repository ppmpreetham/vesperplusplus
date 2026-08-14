use std::convert::Infallible;

use axum::response::{Sse, sse::Event};
use futures_util::Stream;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    todo!()
}

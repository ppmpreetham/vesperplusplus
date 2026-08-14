use crate::api::sse::sse_handler;
use axum::{Router, routing::post};

pub async fn router() -> Router {
    Router::new().route("/api", post(sse_handler))
}

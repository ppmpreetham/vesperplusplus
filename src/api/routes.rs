use crate::api::sse::sse_handler;
use axum::{Router, routing::post};
use tower_http::cors::{Any, CorsLayer};

pub fn router() -> Router {
    Router::new().route("/api", post(sse_handler)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    )
}

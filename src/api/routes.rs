use axum::{Router, extract::Query, routing::post};

pub async fn router() -> Router {
    Router::new().route("/api", post(post_username))
}

// TODO: use post to check and return SSE
async fn post_username(Query(username): Query<String>) -> &'static str {
    "Username received"
}

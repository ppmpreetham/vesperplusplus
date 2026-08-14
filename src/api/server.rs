use super::routes::router;
use axum::serve;
use tokio::net::TcpListener;

pub async fn serve_api(port: Option<u16>) {
    let port = port.unwrap_or(8080);
    let app = router().await;
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("API Listening at http://{}", addr);
    serve(listener, app)
        .await
        .expect("Server error encountered");
}

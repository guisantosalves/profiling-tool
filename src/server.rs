use axum::{Router, routing::get};

pub async fn start() {
    let app = Router::new().route("/", get(|| async { "Hello From system profiler!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

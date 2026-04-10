use axum::{
    Json,
    Router,
    extract::ws::{ Message, WebSocket, WebSocketUpgrade },
    response::Response,
    routing::get,
};
use tokio::time::{ interval, Duration };
use crate::collector;

pub async fn start() {
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hello From system profiler!" })
        )
        .route("/stats", get(get_stats))
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener
        ::bind("0.0.0.0:3000").await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Failed to serve");
}

async fn get_stats() -> Json<collector::SystemStats> {
    Json(collector::collect())
}

async fn handle_socket(mut socket: WebSocket) {
    let mut tick = interval(Duration::from_secs(1));
    loop {
        tick.tick().await;
        let stats = collector::collect();
        let json = serde_json::to_string(&stats).unwrap();
        if socket.send(Message::Text(json.into())).await.is_err() {
            break;
        }
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_socket)
}

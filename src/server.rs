use axum::{
    Json,
    Router,
    extract::{ Path, ws::{ Message, WebSocket, WebSocketUpgrade } },
    http::StatusCode,
    routing::{get, post},
};
use sysinfo::{ Pid, System };
use tokio::time::{ interval, Duration };
use crate::collector;
use tower_http::services::ServeDir;

pub async fn start() {
    let app = Router::new()
        .route("/stats", get(get_stats))
        .route("/ws", get(ws_handler))
        // O ServeDir recebe o "resto da URL" depois do prefixo e procura o arquivo correspondente na pasta.
        .route("/process/{pid}/kill", post(kill_process_handler))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener
        ::bind("0.0.0.0:3000").await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Failed to serve");
}

async fn get_stats() -> Json<collector::SystemStats> {
    Json(collector::collect())
}

async fn kill_process_handler(Path(pid): Path<u32>) -> StatusCode {
    let mut sys = System::new();

    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    match sys.process(Pid::from_u32(pid)) {
        Some(process) => {
            if process.kill() { StatusCode::OK } else { StatusCode::FORBIDDEN }
        }
        None => StatusCode::NOT_FOUND,
    }
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

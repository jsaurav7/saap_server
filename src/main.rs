use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    routing::get,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct Overlay {
    players: Vec<Player>,
}

#[derive(Serialize)]
struct Player {
    name: String,
    score: u32,
}

async fn handle_socket(mut socket: WebSocket) {
    let overlay = bincode::serialize(&Overlay {
        players: vec![Player {
            name: "naag_raj".to_string(),
            score: 0,
        }],
    })
    .unwrap();

    let _ = socket.send(Message::Binary(overlay)).await;
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async { ws.on_upgrade(handle_socket) }),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

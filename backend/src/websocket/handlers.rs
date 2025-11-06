use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{extract::State, response::Response};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use redis::AsyncCommands;
use serde_json::json;
use sqlx::types::Uuid;
use std::sync::Arc;
use std::time::Duration;

use crate::data::Event;
use crate::state::AppState;
use crate::Result;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&id)?;
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, user_id)))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, user_id: Uuid) {
    let (sender, receiver) = socket.split();

    tokio::spawn(write(sender, Arc::clone(&state), user_id));
    tokio::spawn(read(receiver, Arc::clone(&state), user_id));
}

async fn read(mut receiver: SplitStream<WebSocket>, _state: Arc<AppState>, user_id: Uuid) {
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(msg) => {
                println!("Received from client: {:?}", msg);
                // Optionally handle the message here
            }
            Err(_) => {
                println!("Client disconnected: read");
                return;
            }
        }
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>, state: Arc<AppState>, user_id: Uuid) {
    let mut conn = state.storage.redis.connection().await;
    let key = format!("ws:{}", user_id);

    loop {
        if let Ok(msg) = conn.blpop::<&str, Event>(&key, 0.0).await {
            if let Err(_) = sender.send(msg.into()).await {
                println!("Client disconnected: write");
                return;
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

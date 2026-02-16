use axum::extract::ws::{WebSocket, Message};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::AppState;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "devices")]
    Devices { devices: Vec<serde_json::Value> },
    #[serde(rename = "scan_progress")]
    ScanProgress { scan_id: String, progress: serde_json::Value },
}

pub async fn handle_socket(mut socket: WebSocket, _state: AppState) {
    println!("✓ WebSocket client connected");
    
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    println!("  WS Received: {}", text);
                    
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        match ws_msg {
                            WsMessage::Ping => {
                                let response = serde_json::to_string(&WsMessage::Pong).unwrap();
                                let _ = socket.send(Message::Text(response)).await;
                            }
                            _ => {}
                        }
                    }
                }
                Message::Close(_) => {
                    println!("✗ WebSocket client disconnected");
                    break;
                }
                _ => {}
            }
        }
    }
}
